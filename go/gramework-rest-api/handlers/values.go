package handlers

import (
	"github.com/betabandido/gramework-rest-api/domain"
	"github.com/betabandido/gramework-rest-api/repositories"
	"github.com/gramework/gramework"
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

func (h ValuesHandler) GetValue(ctx *gramework.Context) {
	key := ctx.RouteArg("key")
	value, err := h.valueRepository.Get(key)
	if err != nil {
		ctx.Error("", http.StatusNotFound)
	} else {
		ctx.JSON(value)
	}
}

func (h ValuesHandler) PutValue(ctx *gramework.Context) {
	var value domain.Value
	err := ctx.UnJSON(&value)
	if err != nil {
		ctx.Error(err.Error(), http.StatusBadRequest)
	} else {
		h.valueRepository.Put(value)
	}
}