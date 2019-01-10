package handlers

import (
	"encoding/json"
	"github.com/betabandido/rest-api-perf/go/common/domain"
	"github.com/betabandido/rest-api-perf/go/common/repositories"
	"github.com/go-chi/chi"
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
	key := chi.URLParam(request, "key")
	value, err := h.valueRepository.Get(key)
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