package main

import (
	"github.com/betabandido/chi-rest-api/handlers"
	"github.com/betabandido/chi-rest-api/repositories"
	"github.com/go-chi/chi"
	"net/http"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeInMemoryValueRepository(),
	)

	r := chi.NewRouter()
	r.Get("/api/values/{key}", valuesHandler.GetValue)
	r.Put("/api/values", valuesHandler.PutValue)

	http.ListenAndServe(":8000", r)
}