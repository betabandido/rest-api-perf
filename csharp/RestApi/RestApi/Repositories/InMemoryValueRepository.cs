using System.Collections.Generic;
using RestApi.Domain;

namespace RestApi.Repositories
{
    public class InMemoryValueRepository : IValueRepository
    {
        private readonly Dictionary<string, KeyValue> _values = new Dictionary<string, KeyValue>();
        
        public KeyValue Get(string key)
        {
            return _values[key];
        }

        public void Put(KeyValue value)
        {
            _values[value.Key] = value;
        }
    }
}