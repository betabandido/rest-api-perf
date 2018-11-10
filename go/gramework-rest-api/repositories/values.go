package repositories

import "github.com/betabandido/gramework-rest-api/domain"

type ValueRepository interface {
	Get(key string) (*domain.Value, error)
	Put(value domain.Value)
}