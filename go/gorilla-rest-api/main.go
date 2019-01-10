package main

import (
	"github.com/betabandido/rest-api-perf/go/common/repositories"
	"github.com/betabandido/rest-api-perf/go/gorilla-rest-api/handlers"
	"github.com/gorilla/mux"
	"net/http"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeValueRepository(),
	)

	r := mux.NewRouter()
	r.HandleFunc("/api/values/{key}", valuesHandler.GetValue).Methods("GET")
	r.HandleFunc("/api/values", valuesHandler.PutValue).Methods("PUT")

	http.ListenAndServe(":8000", r)
}