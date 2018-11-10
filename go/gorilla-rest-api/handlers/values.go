package handlers

import (
	"encoding/json"
	"github.com/betabandido/gorilla-rest-api/domain"
	"github.com/betabandido/gorilla-rest-api/repositories"
	"github.com/gorilla/mux"
	"net/http"
)

type ValuesHandler struct {
	valueRepository repositories.ValueRepository
}

func MakeValuesHandler(valueRepository repositories.ValueRepository) ValuesHandler {
	return ValuesHandler{
		valueRepository: valueRepository,
	}
}

func (h ValuesHandler) GetValue(writer http.ResponseWriter, request *http.Request) {
	vars := mux.Vars(request)
	value, err := h.valueRepository.Get(vars["key"])
	if err != nil {
		writer.WriteHeader(http.StatusNotFound)
	} else {
		writer.WriteHeader(http.StatusOK)
		err = json.NewEncoder(writer).Encode(value)
		if err != nil {
			panic(err)
		}
	}
}

func (h ValuesHandler) PutValue(writer http.ResponseWriter, request *http.Request) {
	var value domain.Value
	err := json.NewDecoder(request.Body).Decode(&value)
	if err != nil {
		writer.WriteHeader(http.StatusBadRequest)
	} else {
		h.valueRepository.Put(value)
		writer.WriteHeader(http.StatusOK)
	}
}