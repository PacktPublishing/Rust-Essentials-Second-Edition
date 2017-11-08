// connecting to mongodb and some commands:
// this code is meant only as an example snippet
// it will not compile
// Direct connection to a server. Will not look for other servers in the topology.
let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize client.");

let coll = client.db("media").collection("movies");
coll.insert_one(doc!{ "title" => "Back to the Future" }, None).unwrap();
coll.update_one(doc!{}, doc!{ "director" => "Robert Zemeckis" }, None).unwrap();
coll.delete_many(doc!{}, None).unwrap();

let mut cursor = coll.find(None, None).unwrap();
for result in cursor {
    if let Ok(item) = result {
        if let Some(&Bson::String(ref title)) = item.get("title") {
            println!("title: {}", title);
        }
    }
}