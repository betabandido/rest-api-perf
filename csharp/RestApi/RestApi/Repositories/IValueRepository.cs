using System.Threading.Tasks;
using RestApi.Domain;

namespace RestApi.Repositories
{
    public interface IValueRepository
    {
        Task<KeyValue> GetAsync(string key);
        Task PutAsync(KeyValue value);
    }
}