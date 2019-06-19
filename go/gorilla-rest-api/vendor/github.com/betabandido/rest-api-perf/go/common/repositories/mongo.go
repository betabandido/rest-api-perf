package repositories

import (
	"context"
	"fmt"
	"github.com/betabandido/rest-api-perf/go/common/domain"
	"github.com/mongodb/mongo-go-driver/bson"
	"github.com/mongodb/mongo-go-driver/mongo"
)

type MongoValueRepository struct {
	collection *mongo.Collection
}

func MakeMongoValueRepository() MongoValueRepository {
	client, err := mongo.Connect(context.TODO(), "mongodb://localhost:27017")
	if err != nil {
		panic("cannot connect to mongodb")
	}

	collection := client.Database("test").Collection("values")

	return MongoValueRepository{
		collection: collection,
	}
}

func (r MongoValueRepository) Get(key string) (*domain.Value, error) {
	var value domain.Value

	filter := bson.D{{"key", key}}
	if err := r.collection.FindOne(context.TODO(), filter).Decode(&value); err != nil {
		if err == mongo.ErrNoDocuments {
			return nil, fmt.Errorf("no value for key %s", key)
		}

		panic("error decoding document")
	}

	return &value, nil
}

func (r MongoValueRepository) Put(value domain.Value) {
	_, err := r.collection.InsertOne(context.TODO(), value)
	if err != nil {
		panic("error inserting document")
	}
}
