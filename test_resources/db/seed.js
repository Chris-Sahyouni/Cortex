
db = db.getSiblingDB("hosts_db");


const seed_data = [
   {
       "id": "test_1",
       "gpus": 1,
       "make": "NVIDIA",
       "available": true
   },
   {
       "id": "test_2",
       "gpus": 2,
       "make": "NVIDIA",
       "available": true
   },
   {
       "id": "test_3",
       "gpus": 1,
       "make": "AMD",
       "available": true
   }
];

result = db.hosts.insert(seed_data);

console.log("DB seed result: ", result);
