package repositories

import "github.com/betabandido/chi-rest-api/domain"

type ValueRepository interface {
	Get(key string) (*domain.Value, error)
	Put(value domain.Value)
}