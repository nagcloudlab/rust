
// Trader

struct Trader {
    name: String,
    city: String,
}

struct Transaction<'lt>{
     trader: &'lt Trader,
     year: i32,
     value: i32,
}


fn main(){

     
     // create 4 traders

     let trader1 = Trader{
         name: "John".to_string(),
         city: "London".to_string(),
     };

     let trader2 = Trader{
         name: "Robert".to_string(),
         city: "New York".to_string(),
     };

     let trader3 = Trader{
         name: "David".to_string(),
         city: "London".to_string(),
     };

     let trader4 = Trader{
         name: "Peter".to_string(),
         city: "Paris".to_string(),
     };

     // create vector of transactions , count 7

     let transactions = vec![
         Transaction{trader: &trader1, year: 2011, value: 300},
         Transaction{trader: &trader2, year: 2012, value: 1000},
         Transaction{trader: &trader3, year: 2011, value: 400},
         Transaction{trader: &trader4, year: 2012, value: 710},
         Transaction{trader: &trader1, year: 2012, value: 700},
         Transaction{trader: &trader3, year: 2012, value: 950},
         Transaction{trader: &trader2, year: 2012, value: 1000},
     ];

     // // Query 1: Find all transactions from year 2012 and sort them by value (small to high).
     // // Query 2: What are all the unique cities where the traders work?
      // Query 3: Find all traders from London and sort them by name.
      // Query 5: Are there any trader based in London?
      // Query 6: Update all transactions so that the traders from London are set to Hyd.





}