package main

import (
	"github.com/betabandido/gin-rest-api/handlers"
	"github.com/betabandido/gin-rest-api/repositories"
	"github.com/gin-gonic/gin"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeInMemoryValueRepository(),
	)

	r := gin.New()
	r.GET("/api/values/:key", valuesHandler.GetValue)
	r.PUT("/api/values", valuesHandler.PutValue)

	r.Run(":8000")
}