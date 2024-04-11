> [!IMPORTANT]
> Breadcrumbs has recently been rewritten from scratch, and is available in the V4 beta (downloadable via the Obsidian BRAT plugin).
>
> Also, Breadcrumbs now has its own Obsidian Publish docs site!✨ Going forward, that's where you can find all detailed documentation on the plugin. Check it out here: https://publish.obsidian.md/breadcrumbs-docs

Breadcrumbs is an Obsidian plugin that lets you add _directed links_ to your notes, then view/traverse that structure in various ways. Internally, Breadcrumbs uses a graph to represent this structure (much like the regular Obsidian graph, except now, links have _direction_ to them). You tell Breadcrumbs about the structure of your notes, it builds this directed graph, and then lets you visualise and navigate the graph.

# Media

Media related to Breacrumbs. Thanks to everyone for sharing!

## Videos

-   @SkepticMystic: [Breadcrumbs - Everything you need to know](https://www.youtube.com/watch?v=N4QmszBRu9I&pp=ygUUYnJlYWRjcnVtYnMgb2JzaWRpYW4%3D) (Outdated)
-   @SkepticMystic: [Breadcrumbs - Obsidian Community Showcase](https://www.youtube.com/watch?v=DXXB7fHcArg&pp=ygUUYnJlYWRjcnVtYnMgb2JzaWRpYW4%3D) (Outdated)
-   @Zen Productivist: [Threading Mode with the Breadcrumbs Plugin in Obsidian](https://www.youtube.com/watch?v=AS5Mv6YNmsQ) (2022-01-01)

## Written

-   @Rhoadey: [How a Hierarchy Note sharpened my thinking in 20 minutes](https://medium.com/obsidian-observer/how-a-hierarchy-note-sharpened-my-thinking-in-20-minutes-f1c65945f41e?sk=64f4d1f889ff8a99009a060a24778a7f)
-   [Obsidian Hub - Breadcrumbs Quickstart Guide](https://publish.obsidian.md/hub/04+-+Guides%2C+Workflows%2C+%26+Courses/Guides/Breadcrumbs+Quickstart+Guide)
-   [Obsidian Hub - Breadcrumbs for Comparative Law](https://publish.obsidian.md/hub/03+-+Showcases+%26+Templates/Plugin+Showcases/Breadcrumbs+for+Comparative+Law)
-   [Obsidian Hub - How to get the most out of Breadcrumbs](https://publish.obsidian.md/hub/04+-+Guides%2C+Workflows%2C+%26+Courses/Guides/How+to+get+the+most+out+of+the+Breadcrumbs+plugin)

# Feed my coffee problem

If you're so inclined, you can buy me a coffee over here: https://ko-fi.com/skepticmystic :)

# Contributing

## Release process

### Prod

TODO

### Beta

1. Push all previous changes to the actual project
2. Bump version in `package.json`
3. `npm run version:beta` to update `manifest-beta.json` and `versions.json` accordingly
4. `git tag -a x.x.x-beta -m 'x.x.x-beta'` to tag the build
5. `git push origin x.x.x-beta` to push the release and trigger the action

Or, do steps 3-5 in one go with `npm run release:beta`
