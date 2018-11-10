package main

import (
	"github.com/betabandido/gorilla-rest-api/handlers"
	"github.com/betabandido/gorilla-rest-api/repositories"
	"github.com/gorilla/mux"
	"net/http"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeInMemoryValueRepository(),
	)

	r := mux.NewRouter()
	r.HandleFunc("/api/values/{key}", valuesHandler.GetValue).Methods("GET")
	r.HandleFunc("/api/values", valuesHandler.PutValue).Methods("PUT")

	http.ListenAndServe(":8000", r)
}