package main

import (
	"github.com/apex/log"
	"github.com/apex/log/handlers/cli"
	"github.com/betabandido/gramework-rest-api/handlers"
	"github.com/betabandido/gramework-rest-api/repositories"
	"github.com/gramework/gramework"
	"os"
)

func main() {
	valuesHandler := handlers.MakeValuesHandler(
		repositories.MakeInMemoryValueRepository(),
	)

	app := gramework.New()

	gramework.SetEnv(gramework.PROD)

	app.Logger = &log.Logger{
		Level: log.ErrorLevel,
		Handler: cli.New(os.Stdout),
	}

	app.GET("/api/values/:key", valuesHandler.GetValue)
	app.PUT("/api/values", valuesHandler.PutValue)

	app.ListenAndServe(":8000")
}