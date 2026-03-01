package main

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"path"
	"regexp"
	"strings"
	"text/template"

	"github.com/gomarkdown/markdown"
	"github.com/gomarkdown/markdown/html"
	"github.com/gomarkdown/markdown/parser"
)

const TemplateName string = "HtmlPage"
const ContentDir string = "contents/"

type HtmlPage struct {
	Intro   string
	Tagline string
	Corpus  string
}

func GetTemplateBuilder() (*template.Template, error) {
	content, err := os.ReadFile("template.html")
	if err != nil {
		return nil, err
	}
	return template.Must(template.New(TemplateName).Parse(string(content))), nil
}

func GetRegex() *regexp.Regexp {
	re := regexp.MustCompile(`---\s*intro:\s*(.*)\s*tagline:\s*(.*)\s*---`)
	return re
}

func GetParser() *parser.Parser {
	extensions := parser.CommonExtensions | parser.NoEmptyLineBeforeBlock
	p := parser.NewWithExtensions(extensions)
	return p
}

func GetRenderer() *html.Renderer {
	htmlFlags := html.CommonFlags | html.HrefTargetBlank
	opts := html.RendererOptions{Flags: htmlFlags}
	renderer := html.NewRenderer(opts)
	return renderer
}

func MarkdownToPage(filePath string, pattern *regexp.Regexp, renderer *html.Renderer) (*HtmlPage, error) {
	parser := GetParser()
	content, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}
	matches := pattern.FindStringSubmatch(string(content))
	if len(matches) >= 3 {
		intro := matches[1]
		tagline := matches[2]
		parts := strings.SplitN(string(content), "---", 3)
		if len(parts) == 3 {
			doc := parser.Parse([]byte(parts[2]))
			corpus := string(markdown.Render(doc, renderer))
			return &HtmlPage{Intro: intro, Tagline: tagline, Corpus: corpus}, nil
		}
		return nil, fmt.Errorf("Expected the document to be divided into three parts, got %d", len(parts))
	}
	return nil, fmt.Errorf("Expected the document to yield matches of a length of at least 3, got %d", len(matches))
}

func GetAllFiles() ([]string, error) {
	files, err := os.ReadDir(ContentDir)
	if err != nil {
		return nil, err
	}
	fls := make([]string, 0, len(files))
	for _, file := range files {
		fl := path.Join(ContentDir, file.Name())
		fls = append(fls, fl)
	}
	return fls, nil
}

func GetFileHtmlPath(fl string) string {
	basePath := path.Base(fl)
	replacedPath := strings.ReplaceAll(basePath, ".md", ".html")
	fullPath := path.Join("../docs/", replacedPath)
	return fullPath
}

func main() {
	files, err := GetAllFiles()
	if err != nil {
		log.Printf("An error occurred while getting the files to transform: %s\n", err.Error())
		os.Exit(1)
	}
	pattern := GetRegex()
	renderer := GetRenderer()
	template, err := GetTemplateBuilder()
	if err != nil {
		log.Printf("An error occurred while getting the HTML template: %s\n", err.Error())
		os.Exit(2)
	}
	for _, file := range files {
		htmlPage, err := MarkdownToPage(file, pattern, renderer)
		if err != nil {
			log.Printf("An error occurred while converting file %s to HTML: %s\n", file, err.Error())
			os.Exit(3)
		}
		var buf bytes.Buffer
		err = template.Execute(&buf, htmlPage)
		if err != nil {
			log.Printf("An error occurred while rendering the template for file %s: %s\n", file, err.Error())
			os.Exit(4)
		}
		content := buf.Bytes()
		filePath := GetFileHtmlPath(file)
		err = os.WriteFile(filePath, content, 0644)
		if err != nil {
			log.Printf("An error occurred while writing HTML content to file %s: %s\n", filePath, err.Error())
			os.Exit(5)
		}
		log.Printf("Conversion for file %s successfully finished, find the converted file at: %s\n", file, filePath)
	}
	log.Println("Everything done!")
	os.Exit(0)
}
