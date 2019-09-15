using System.Collections.Generic;
using System.Threading.Tasks;
using RestApi.Domain;

namespace RestApi.Repositories
{
    public class InMemoryValueRepository : IValueRepository
    {
        private readonly Dictionary<string, KeyValue> _values = new Dictionary<string, KeyValue>();
        
        public Task<KeyValue> GetAsync(string key)
        {
            return Task.FromResult(_values[key]);
        }

        public Task PutAsync(KeyValue value)
        {
            _values[value.Key] = value;
            
            return Task.CompletedTask;
        }
    }
}