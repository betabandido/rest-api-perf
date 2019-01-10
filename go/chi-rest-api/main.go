package main

import (
	"github.com/betabandido/rest-api-perf/go/chi-rest-api/handlers"
	"github.com/betabandido/rest-api-perf/go/common/repositories"
	"github.com/go-chi/chi"
	"net/http"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeValueRepository(),
	)

	r := chi.NewRouter()
	r.Get("/api/values/{key}", valuesHandler.GetValue)
	r.Put("/api/values", valuesHandler.PutValue)

	http.ListenAndServe(":8000", r)
}