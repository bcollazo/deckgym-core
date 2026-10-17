import fs from 'node:fs'
import path from 'node:path'
import react from '@vitejs/plugin-react'
import { defineConfig, type Connect, type Plugin } from 'vite'

// Serves the repo-root `replays/` folder (one level up from `viewer/`) at `/replays/<file>.json` —
// what `cargo run -- play`'s printed link (`?url=/replays/<game_id>.json`) needs to actually
// resolve while `npm run dev`/`npm run preview` is running. See docs/replay-viewer-plan.md's
// "Round 4" notes and the root README's "Replay viewer" section for the end-to-end flow.
//
// Deliberately narrow: only `/replays/<one path segment>.json` is served (no subdirectories, no
// other extensions, no `..`), and only from that one fixed folder — this is a local dev
// convenience, not a general static file server.
function replaysStaticPlugin(): Plugin {
  const replaysDir = path.resolve(import.meta.dirname, '..', 'replays')

  const middleware: Connect.NextHandleFunction = (req, res, next) => {
    const url = req.url ?? ''
    if (!url.startsWith('/replays/')) {
      next()
      return
    }

    const rawPath = url.split('?')[0].split('#')[0]
    let rel: string
    try {
      rel = decodeURIComponent(rawPath.slice('/replays/'.length))
    } catch {
      res.statusCode = 400
      res.end('Invalid replay path')
      return
    }

    // Exactly one path segment, ending in .json — rejects traversal (`..`, more `/`) outright
    // rather than trying to sanitize it, and rejects anything but the one file type replays are.
    if (!/^[^/\\]+\.json$/.test(rel) || rel.includes('..')) {
      res.statusCode = 400
      res.end('Invalid replay path')
      return
    }

    const resolved = path.resolve(replaysDir, rel)
    if (resolved !== path.join(replaysDir, rel) || !resolved.startsWith(replaysDir + path.sep)) {
      res.statusCode = 400
      res.end('Invalid replay path')
      return
    }

    fs.readFile(resolved, (err, data) => {
      if (err) {
        res.statusCode = 404
        res.end('Replay not found')
        return
      }
      res.setHeader('Content-Type', 'application/json')
      res.end(data)
    })
  }

  return {
    name: 'deckgym-replays-static',
    configureServer(server) {
      server.middlewares.use(middleware)
    },
    configurePreviewServer(server) {
      server.middlewares.use(middleware)
    },
  }
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), replaysStaticPlugin()],
})
