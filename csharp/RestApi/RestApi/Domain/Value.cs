using MongoDB.Bson.Serialization.Attributes;

namespace RestApi.Domain
{
    [BsonIgnoreExtraElements]
    public class KeyValue
    {
        [BsonElement("key")]
        public string Key { get; set; }
        
        [BsonElement("value")]
        public string Value { get; set; }
    }
}