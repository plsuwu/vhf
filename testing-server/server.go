package main

import "github.com/gin-gonic/gin"

// i have no fucking idea how go works lmao

func main() {
    app := gin.Default()
    router := app.Group("/")
    router.GET("/", Hello)
    router.GET("/test", Test)
    app.Run(":8080")
}


func Hello(c *gin.Context) {
    c.Writer.Write([]byte("<h1> omg hiii </h1>"))
}

func Test(c *gin.Context) {
    c.Writer.Write([]byte("<h1> test lmao </h1>"))
}
