## Generating api route docs

If you want to know all routes available in this application, it really simple,
first of all, make sure you have got *nodejs* and *npm* properly installed on your 
machine, then run the following command: 

```bash
npx insomnia-documenter --config ./routes.json
```

Then, you can run a server with your brand new doc by using:
```bash
npx serve
```

Now it's as simple as opening a new tab on your favourite browser and accessing the link:
> http://localhost:3000
