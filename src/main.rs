// use am::{transaction::CommitOptions, AutomergeError, ROOT};
use anyhow::Result;
// use automerge::{self as am, transaction::Transactable, Automerge, ObjType};

mod value;

fn main() -> Result<()> {
    value::fake_main()
    /*
    let mut doc1 = Automerge::new();

    let (cards, card1, _card2) = doc1
        .transact_with::<_, _, AutomergeError, _, ()>(
            |_| CommitOptions::default().with_message("Add card".to_owned()),
            |tx| {
                let cards = tx.put_object(ROOT, "cards", ObjType::List)?;
                let card1 = tx.insert_object(&cards, 0, ObjType::Map)?;
                tx.put(&card1, "title", "Rewrite everything in Clojure")?;
                tx.put(&card1, "done", false)?;
                let card2 = tx.insert_object(&cards, 0, ObjType::Map)?;
                tx.put(&card2, "title", "Rewrite everything in Haskell")?;
                tx.put(&card2, "done", false)?;
                Ok((cards, card1, card2))
            },
        )
        .map(|s| s.result)
        .map_err(|e| e.error)?;

    let mut doc2 = Automerge::new();
    doc2.merge(&mut doc1).unwrap();

    let binary = doc1.save();
    let mut doc3 = Automerge::load(&binary).unwrap();

    doc1.transact_with::<_, _, AutomergeError, _, ()>(
        |_| CommitOptions::default().with_message("Mark card as done #1".to_owned()),
        |tx| {
            tx.put(&card1, "done", true)?;
            Ok(())
        },
    )
    .unwrap();
    doc1.transact_with::<_, _, AutomergeError, _, ()>(
        |_| CommitOptions::default().with_message("Mark card as done #2".to_owned()),
        |tx| {
            tx.put(&card1, "done", true)?;
            Ok(())
        },
    )
    .unwrap();
    doc1.transact_with::<_, _, AutomergeError, _, ()>(
        |_| CommitOptions::default().with_message("Mark card as done #3".to_owned()),
        |tx| {
            tx.put(&card1, "done", true)?;
            Ok(())
        },
    )
    .unwrap();

    doc3.transact_with::<_, _, AutomergeError, _, ()>(
        |_| CommitOptions::default().with_message("Delete card".to_owned()),
        |tx| {
            tx.delete(&cards, 0)?;
            Ok(())
        },
    )
    .unwrap();

    doc1.merge(&mut doc3).unwrap();

    for change in doc1.get_changes(&[]).unwrap() {
        println!("{}", change.message().unwrap());
    }

    println!("{:?}", doc1.get(card1, "done").unwrap().unwrap().0);
    println!("{:?}", doc1.length(ROOT));
    println! {"{:?}", doc1.values(ROOT).into_iter().collect::<Vec<_>>()}

    Ok(())
    */
}
