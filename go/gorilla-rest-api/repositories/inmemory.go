package repositories

import (
	"fmt"
	"github.com/betabandido/gorilla-rest-api/domain"
)

type InMemoryValueRepository struct {
	values map[string]domain.Value
}

func MakeInMemoryValueRepository() InMemoryValueRepository {
	return InMemoryValueRepository{
		values: map[string]domain.Value{},
	}
}

func (r InMemoryValueRepository) Get(key string) (*domain.Value, error) {
	if value, ok := r.values[key]; ok {
		return &value, nil
	}

	return nil, fmt.Errorf("no value for key %s", key)
}

func (r InMemoryValueRepository) Put(value domain.Value) {
	r.values[value.Key] = value
}