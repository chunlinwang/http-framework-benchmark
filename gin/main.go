package main

import ( 
	"github.com/gin-gonic/gin"
	"io/fs"
	"os"
	"net/http"
)

func main() {
	r := gin.Default()

	r.GET("/test", func(c *gin.Context) {
		data, _ := fs.ReadFile(os.DirFS("."), "example_849K.json")
		c.JSONP(http.StatusOK, string(data))
	})

	r.Run() // 监听并在 0.0.0.0:8080 上启动服务
}

