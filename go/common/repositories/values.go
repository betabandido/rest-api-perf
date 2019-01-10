package repositories

import (
	"fmt"
	"github.com/betabandido/rest-api-perf/go/common/domain"
	"os"
)

type ValueRepository interface {
	Get(key string) (*domain.Value, error)
	Put(value domain.Value)
}

func MakeValueRepository() ValueRepository {
	repoType := os.Getenv("REST_API_PERF_REPO")

	switch repoType {
	case "mongo":
		return MakeMongoValueRepository()
	case "in-memory":
		return MakeInMemoryValueRepository()
	default:
		panic(fmt.Sprintf("invalid repository type: %s", repoType))
	}
}
