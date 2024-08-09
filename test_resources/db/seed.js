
db = db.getSiblingDB("hosts_db");
// db.createCollection("hosts");


const seed_data = [
   {
       "id": "test_1",
       "gpus": 1,
       "model": "<model_name>",
       "make": "NVIDIA",
       "available": true
   },
   {
       "id": "test_2",
       "gpus": 2,
       "model": "<model_name>",
       "make": "NVIDIA",
       "available": true
   },
   {
       "id": "test_3",
       "gpus": 1,
       "model": "<model_name>",
       "make": "AMD",
       "available": true
   }
];

result = db.hosts.insert(seed_data);

console.log("DB seed result: ", result);
