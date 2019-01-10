using System.Linq;
using MongoDB.Driver;
using RestApi.Domain;

namespace RestApi.Repositories
{
    public class MongoValueRepository : IValueRepository
    {
        private readonly IMongoCollection<KeyValue> _collection;

        public MongoValueRepository(IMongoClient client)
        {
            _collection = client.GetDatabase("test").GetCollection<KeyValue>("values");
        }
        
        public KeyValue Get(string key)
        {
            var results = _collection
                .Find(x => x.Key == key)
                .ToList();

            return results.Count == 0
                ? null
                : results.First();
        }

        public void Put(KeyValue value)
        {
            _collection.InsertOne(value);
        }
    }
}