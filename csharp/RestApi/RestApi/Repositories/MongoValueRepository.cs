using System.Threading.Tasks;
using MongoDB.Driver;
using RestApi.Domain;

namespace RestApi.Repositories
{
    public class MongoValueRepository : IValueRepository
    {
        private readonly IMongoCollection<KeyValue> _collection;

        public MongoValueRepository(IMongoClient client)
        {
            _collection = client
                .GetDatabase("test")
                .GetCollection<KeyValue>("values");
        }
        
        public async Task<KeyValue> GetAsync(string key)
        {
            var cursor = await _collection.FindAsync(x => x.Key == key);

            return await cursor.FirstOrDefaultAsync();
        }

        public async Task PutAsync(KeyValue value)
        {
            await _collection.InsertOneAsync(value);
        }
    }
}