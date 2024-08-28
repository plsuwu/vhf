package main
import "github.com/gin-gonic/gin"
func main() {
    app := gin.Default()
    router := app.Group("/")
    router.GET("/", Hello)
    app.Run(":8080")
}


func Hello(c *gin.Context) {
    c.Writer.Write([]byte("<h1> omg hiii </h1>"))
}
