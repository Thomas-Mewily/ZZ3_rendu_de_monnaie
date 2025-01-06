use std::fmt::Debug;

type Euro = i64;
type Qte  = i64;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Monnaie
{
    valeur   : Euro,
    quantite : Qte,
}
impl Debug for Monnaie
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{} euro x{}", self.valeur, self.quantite) }
}
pub trait MonnaieExtension where Self : Sized
{ 
    fn euro_x(self, qte : Qte) -> Monnaie;
    fn euro_big_qte(self) -> Monnaie { self.euro_x(1000) }
}
impl MonnaieExtension for Euro { fn euro_x(self, qte : Qte) -> Monnaie { Monnaie { valeur: self, quantite: qte } } }

/// `posseder` : un vecteur de monnaie, où chaque élément à une la valeur positive ou nulle 
/// Retourne nn vecteur de monnaie à rendre, ou une erreur si l'opération n'est pas possible
pub fn rendu_de_monnaie(a_rendre : Euro, posseder : &Vec<Monnaie>) -> Result<Vec<Monnaie>, &'static str>
{
    fn _rendu_de_monnaie(a_rendre : Euro, posseder : &Vec<Monnaie>, depenser : &mut Vec<Qte>, idx : usize, qte_rendu : &mut Qte, meilleur_qte_rendu : &mut Euro, meilleur_depenser : &mut Vec<Qte>) -> bool
    {
        assert!(a_rendre >= 0);
        if a_rendre == 0 
        {
            if *qte_rendu <= *meilleur_qte_rendu
            {
                *meilleur_qte_rendu = *qte_rendu;
                *meilleur_depenser = depenser.clone();
            }
            return true;
        }

        let Some(v) = posseder.get(idx) else { return false; };

        let qte_a_donner = (a_rendre / v.valeur).min(v.quantite);

        let mut trouver_une_soluce = false;
        let qte_rendu_actuel = *qte_rendu;
        // C'est possiblede faire une opti avec la valeur cumulé croissant/décroissant de posseder
        for qte in (0..=qte_a_donner).rev()
        {
            *qte_rendu = qte_rendu_actuel + qte;
            depenser[idx] = qte;
            
            if *qte_rendu >= *meilleur_qte_rendu { continue }
            trouver_une_soluce |= _rendu_de_monnaie(a_rendre - qte * v.valeur, posseder, depenser, idx + 1, qte_rendu, meilleur_qte_rendu, meilleur_depenser);
        }
        *qte_rendu = qte_rendu_actuel;
        trouver_une_soluce
    }

    if a_rendre == 0 { return Ok(vec![]) }
    if a_rendre <  0 { return Err("C'est à toi de me donner de l'argent"); }

    let mut posseder_filtrer : Vec<Monnaie> = Vec::with_capacity(posseder.len());

    // Merge les entrées dont les valeurs sont identiques, et les tries par ordre croissant
    let mut total_caisse = 0;
    let mut total_qte_caisse = 0;
    for m in posseder
    {
        if m.valeur == 0 || m.quantite == 0 { continue; }

        if m.valeur   < 0 { Err("On n'a pas d'argent négatif dans la caisse (piece/billet de valeur négative)")? }
        if m.quantite < 0 { Err("L'antimatière c'est pas pour aujourd'hui (quantité négative d'argent)")?        }

        match posseder_filtrer.binary_search_by(|e| m.valeur.cmp(&e.valeur))
        {
            Ok(idx) => posseder_filtrer[idx].quantite += m.quantite,
            Err(idx) => posseder_filtrer.insert(idx, *m),
        }

        total_caisse += m.valeur * m.quantite;
        total_qte_caisse += m.quantite;
    }

    if total_caisse < a_rendre { return Err("On a pas assez d'argent dans la caisse"); }

    let mut depenser = vec![0; posseder.len()];
    let mut meilleur_depenser = vec![0; posseder.len()];

    let mut meilleur_qte_rendu = total_qte_caisse + 1;
    let mut qte_rendu = 0;
    
    _rendu_de_monnaie(a_rendre, &posseder_filtrer, &mut depenser, 0, &mut qte_rendu, &mut meilleur_qte_rendu, &mut meilleur_depenser);//
    
    if meilleur_qte_rendu != total_qte_caisse + 1
    {
        Ok(meilleur_depenser.into_iter().enumerate().map(|(idx, quantite)| Monnaie { valeur: posseder_filtrer[idx].valeur, quantite }).filter(|e| e.quantite > 0).collect())
    }else
    {
        Err("impossible")
    }
}


fn print_rendu_de_monnaie(rendu : Euro, posseder : Vec<Monnaie>) -> Result<Vec<Monnaie>, &'static str>
{
    let r = rendu_de_monnaie(rendu, &posseder);
    match &r
    {
        Ok(qte) => println!("Pour rendre {} euro avec {:?} il faut {:?}", rendu, posseder, qte),
        Err(e) => println!("impossible de rendre {} euro avec {:?} : {}", rendu, posseder, e),
    }
    println!();
    r
}

// Adapté pour tester le code sur leetcode : Coin Change, l'énnoncé est un peu différent
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 
{
    let coins = coins.iter().map(|valeur| Monnaie { valeur : *valeur as Euro, quantite:9999999 }).collect();
    rendu_de_monnaie(amount as Euro, &coins).map(|v| v.iter().map(|e| e.quantite).sum::<Qte>() as i32).unwrap_or(-1)
}

fn main() 
{
    assert!(print_rendu_de_monnaie(25 , vec![1.euro_x(5), 10.euro_x(1), 2.euro_x(5)]).is_ok());
    assert!(print_rendu_de_monnaie(200, vec![10.euro_x(1), 1.euro_x(5), 2.euro_x(5)]).is_err());

    assert!(print_rendu_de_monnaie(200, vec![10.euro_x(1), 1.euro_x(5), 2.euro_x(5)]).is_err());

    assert!(print_rendu_de_monnaie(50, vec![0.euro_x(50)]).is_err());
    assert!(print_rendu_de_monnaie(50, vec![1.euro_x(500), (-5).euro_x(50)]).is_err());

    assert!(print_rendu_de_monnaie(1_000_000_000_001, vec![1.euro_x(1_000_000_000_000)]).is_err());

    assert_eq!(print_rendu_de_monnaie(28, vec![20.euro_x(1), 1.euro_x(10), 14.euro_x(5)]).ok().unwrap(), [Monnaie{ valeur: 14, quantite: 2 }]);

    assert!(print_rendu_de_monnaie(6249 , vec![186.euro_big_qte(), 419.euro_big_qte(), 83.euro_big_qte(), 408.euro_big_qte()]).is_ok());
}
