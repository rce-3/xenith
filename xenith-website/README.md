# Xenith website

This serves as Xenith project's official website, documentation and blog.

It is powered by [Hugo](https://gohugo.io/) and [Hextra](https://imfing.github.io/hextra/) theme. It is dynamically published to Github Pages.

## Local Development

Pre-requisites: [Hugo](https://gohugo.io/getting-started/installing/), [Go](https://golang.org/doc/install) and [Git](https://git-scm.com)

```shell
# Clone the repo
git clone git@github.com:rce-3/xenith.git

# Change directory
cd xenith

# Start the server
hugo server --logLevel debug --buildDrafts --disableFastRender --source xenith-website/
```

You can access the website at [http://localhost:1313](http://localhost:1313).

### Update theme

```shell
hugo mod get -u
hugo mod tidy
```

See [Update modules](https://gohugo.io/hugo-modules/use-modules/#update-modules) for more details.
