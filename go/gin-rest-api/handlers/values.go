package handlers

import (
	"github.com/betabandido/gin-rest-api/domain"
	"github.com/betabandido/gin-rest-api/repositories"
	"github.com/gin-gonic/gin"
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

func (h ValuesHandler) GetValue(ctx *gin.Context) {
	value, err := h.valueRepository.Get(ctx.Param("key"))
	if err != nil {
		ctx.String(http.StatusNotFound, "")
	} else {
		ctx.JSON(http.StatusOK, value)
	}
}

func (h ValuesHandler) PutValue(ctx *gin.Context) {
	var value domain.Value
	ctx.BindJSON(&value)
	h.valueRepository.Put(value)
	ctx.String(http.StatusOK, "")
}