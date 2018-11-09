using RestApi.Domain;

namespace RestApi.Repositories
{
    public interface IValueRepository
    {
        KeyValue Get(string key);
        void Put(KeyValue value);
    }
}