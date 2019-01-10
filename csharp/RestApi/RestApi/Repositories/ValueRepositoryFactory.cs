using System;
using MongoDB.Driver;

namespace RestApi.Repositories
{
    public static class ValueRepositoryFactory
    {
        public static IValueRepository Create()
        {
            var repoType = Environment.GetEnvironmentVariable("REST_API_PERF_REPO");
            
            switch (repoType)
            {
                case "mongo":
                    const string connectionString = "mongodb://localhost:27017";
                    var client = new MongoClient(connectionString);
                    return new MongoValueRepository(client);
                case "in-memory":
                    return new InMemoryValueRepository();
                default:
                    throw new ArgumentException(
                        $"invalid repository type: {repoType}"
                    );
            }
        }
    }
}