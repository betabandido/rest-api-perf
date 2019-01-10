package main

import (
	"github.com/betabandido/rest-api-perf/go/common/repositories"
	"github.com/betabandido/rest-api-perf/go/gin-rest-api/handlers"
	"github.com/gin-gonic/gin"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeValueRepository(),
	)

	r := gin.New()
	r.GET("/api/values/:key", valuesHandler.GetValue)
	r.PUT("/api/values", valuesHandler.PutValue)

	r.Run(":8000")
}
