# Directory Structure Report

This document contains files from the `babysitter` directory with extensions: js, ts, py
Custom ignored patterns: specializations
Content hash: 291ab66f088ca739

## File Tree Structure

- 📁 packages
  - 📁 babysitter
    - 📁 bin
      - 📄 babysitter.js
  - 📁 breakpoints
    - 📁 api
      - 📄 db.js
      - 📄 routes.js
      - 📄 server.js
    - 📁 bin
      - 📄 breakpoints.js
    - 📁 extensions
      - 📄 config.js
      - 📄 index.js
      - 📄 telegram.js
    - 📁 scripts
      - 📄 dev-runner.js
      - 📄 init-db.js
    - 📁 web
      - 📄 app.js
      - 📄 server.js
    - 📁 worker
      - 📄 queue.js
      - 📄 worker.js
  - 📁 catalog
    - 📄 eslint.config.mjs
    - 📄 next.config.ts
    - 📄 postcss.config.mjs
    - 📄 process-library-catalog.js
    - 📁 scripts
      - 📄 reindex.ts
    - 📁 src
      - 📁 app
        - 📁 agents
          - 📁 [slug]
            - 📄 page.tsx
          - 📄 loading.tsx
          - 📄 page.tsx
        - 📁 api
          - 📁 agents
            - 📁 [slug]
              - 📄 route.ts
            - 📄 route.ts
          - 📁 analytics
            - 📄 route.ts
          - 📁 domains
            - 📁 [slug]
              - 📄 route.ts
            - 📄 route.ts
          - 📁 processes
            - 📁 [id]
              - 📄 route.ts
            - 📄 route.ts
          - 📁 reindex
            - 📄 route.ts
          - 📁 search
            - 📄 route.ts
          - 📁 skills
            - 📁 [slug]
              - 📄 route.ts
            - 📄 route.ts
        - 📁 domains
          - 📁 [slug]
            - 📄 page.tsx
          - 📄 page.tsx
        - 📄 layout.tsx
        - 📄 page.tsx
        - 📁 processes
          - 📁 [id]
            - 📄 page.tsx
          - 📄 loading.tsx
          - 📄 page.tsx
        - 📁 search
          - 📄 loading.tsx
          - 📄 page.tsx
        - 📁 skills
          - 📁 [slug]
            - 📄 page.tsx
          - 📄 loading.tsx
          - 📄 page.tsx
      - 📁 components
        - 📄 ErrorBoundary.tsx
        - 📁 catalog
          - 📁 DetailView
            - 📄 AgentDetail.tsx
            - 📄 ProcessDetail.tsx
            - 📄 SkillDetail.tsx
            - 📄 index.ts
          - 📁 EntityCard
            - 📄 AgentCard.tsx
            - 📄 DomainCard.tsx
            - 📄 ProcessCard.tsx
            - 📄 SkillCard.tsx
            - 📄 index.ts
          - 📄 EntityList.tsx
          - 📄 FilterPanel.tsx
          - 📄 MetadataDisplay.tsx
          - 📄 QuickActions.tsx
          - 📄 RelatedItems.tsx
          - 📄 SearchBar.tsx
          - 📄 SortDropdown.tsx
          - 📄 TreeView.tsx
          - 📄 index.ts
        - 📁 common
          - 📄 EmptyState.tsx
          - 📄 LoadingSkeleton.tsx
          - 📄 Pagination.tsx
          - 📄 SearchInput.tsx
          - 📄 Tag.tsx
          - 📄 index.ts
        - 📁 dashboard
          - 📄 BarChart.tsx
          - 📄 MetricCard.tsx
          - 📄 PieChart.tsx
          - 📄 QuickLinks.tsx
          - 📄 RecentActivity.tsx
          - 📄 StatsOverview.tsx
          - 📄 TreemapChart.tsx
          - 📄 index.ts
        - 📁 decorations
          - 📄 BotanicalDecor.tsx
          - 📄 BrassPipeBorder.tsx
          - 📄 CardCornerFlourish.tsx
          - 📄 GearCluster.tsx
          - 📄 MechanicalBee.tsx
          - 📄 index.ts
        - 📁 layout
          - 📄 Breadcrumb.tsx
          - 📄 Footer.tsx
          - 📄 Header.tsx
          - 📄 PageContainer.tsx
          - 📄 Sidebar.tsx
          - 📄 index.ts
        - 📁 markdown
          - 📄 CodeBlock.tsx
          - 📄 FrontmatterDisplay.tsx
          - 📄 ImageHandler.tsx
          - 📄 LinkHandler.tsx
          - 📄 MarkdownRenderer.tsx
          - 📄 TableOfContents.tsx
          - 📄 index.ts
        - 📁 ui
          - 📄 badge.tsx
          - 📄 button.tsx
          - 📄 card.tsx
          - 📄 index.ts
          - 📄 input.tsx
          - 📄 separator.tsx
          - 📄 skeleton.tsx
      - 📁 hooks
        - 📄 index.ts
      - 📁 lib
        - 📁 api
          - 📄 index.ts
          - 📄 types.ts
          - 📄 utils.ts
        - 📁 db
          - 📄 client.ts
          - 📄 index.ts
          - 📄 indexer.ts
          - 📄 init.ts
          - 📄 queries.ts
          - 📄 schema.ts
          - 📄 types.ts
        - 📁 hooks
          - 📄 useApi.ts
        - 📁 parsers
          - 📄 agent.ts
          - 📄 ast.ts
          - 📄 directory.ts
          - 📄 frontmatter.ts
          - 📄 index.ts
          - 📄 jsdoc.ts
          - 📄 markdown.ts
          - 📄 process.ts
          - 📄 skill.ts
          - 📄 types.ts
        - 📄 utils.ts
      - 📁 types
        - 📄 index.ts
  - 📁 sdk
    - 📄 .eslintrc.cjs
    - 📁 scripts
      - 📄 smoke-cli.js
    - 📁 src
      - 📁 cli
        - 📁 __tests__
          - 📄 cliMain.test.ts
          - 📄 cliRuns.test.ts
          - 📄 cliTasks.test.ts
        - 📁 commands
          - 📄 runIterate.ts
        - 📄 completionSecret.ts
        - 📄 main.ts
      - 📁 hooks
        - 📄 dispatcher.ts
        - 📄 index.ts
        - 📄 types.ts
      - 📄 index.ts
      - 📁 runtime
        - 📁 __tests__
          - 📄 commitEffectResult.test.ts
          - 📄 createReplayEngine.test.ts
          - 📄 createRun.test.ts
          - 📄 deterministicHarness.test.ts
          - 📄 effectActions.contract.test.ts
          - 📄 effectIndex.test.ts
          - 📄 exceptions.test.ts
          - 📄 intrinsics.behaviors.test.ts
          - 📄 orchestrateIteration.integration.test.ts
          - 📄 parallel.test.ts
          - 📄 processContext.test.ts
          - 📄 replayCursor.test.ts
          - 📄 taskIntrinsic.test.ts
          - 📄 testHelpers.ts
        - 📄 commitEffectResult.ts
        - 📄 constants.ts
        - 📄 createRun.ts
        - 📄 errorUtils.ts
        - 📄 exceptions.ts
        - 📁 hooks
          - 📄 runtime.ts
        - 📄 index.ts
        - 📄 instrumentation.ts
        - 📁 intrinsics
          - 📄 breakpoint.ts
          - 📄 hook.ts
          - 📄 index.ts
          - 📄 orchestratorTask.ts
          - 📄 parallel.ts
          - 📄 sleep.ts
          - 📄 task.ts
        - 📁 invocation
          - 📄 hashInvocationKey.ts
          - 📄 index.ts
        - 📄 orchestrateIteration.ts
        - 📄 processContext.ts
        - 📁 replay
          - 📁 __tests__
            - 📄 stateCache.test.ts
          - 📄 createReplayEngine.ts
          - 📄 effectIndex.ts
          - 📄 index.ts
          - 📄 replayCursor.ts
          - 📄 stateCache.ts
        - 📄 types.ts
      - 📁 storage
        - 📁 __tests__
          - 📄 storage.test.ts
        - 📄 atomic.ts
        - 📄 cleanup.ts
        - 📄 clock.ts
        - 📄 createRunDir.ts
        - 📄 index.ts
        - 📄 journal.ts
        - 📄 lock.ts
        - 📄 paths.ts
        - 📄 runFiles.ts
        - 📄 snapshotState.ts
        - 📄 storeTaskArtifacts.ts
        - 📄 tasks.ts
        - 📄 types.ts
        - 📄 ulids.ts
      - 📁 tasks
        - 📁 __tests__
          - 📄 batching.test.ts
          - 📄 context.test.ts
          - 📄 defineTask.test.ts
          - 📄 kinds.test.ts
          - 📄 registry.test.ts
          - 📄 serializer.test.ts
        - 📄 batching.ts
        - 📄 context.ts
        - 📄 defineTask.ts
        - 📄 index.ts
        - 📁 kinds
          - 📄 index.ts
        - 📄 registry.ts
        - 📄 serializer.ts
        - 📄 types.ts
      - 📁 test-fixtures
        - 📁 kinds
          - 📄 index.ts
      - 📁 testing
        - 📁 __tests__
          - 📄 parallelHarness.test.ts
          - 📄 runHarness.test.ts
        - 📄 deterministic.ts
        - 📄 index.ts
        - 📄 runHarness.ts
        - 📄 snapshots.ts
    - 📁 test-fixtures
      - 📁 cli
        - 📁 processes
          - 📄 smoke-node-runner.mjs
          - 📄 smoke-process.mjs
      - 📁 kinds
        - 📄 index.d.ts
        - 📄 index.js
        - 📄 index.ts
      - 📁 runner
        - 📄 copy-inputs-to-output.js
        - 📄 emit-logs.js
        - 📄 print-env.js
        - 📄 slow-logger.js
    - 📄 vitest.config.ts
  - 📁 vscode-extension
    - 📄 .eslintrc.cjs
    - 📁 scripts
      - 📄 run-vsce.cjs
    - 📁 src
      - 📁 core
        - 📄 awaitingInput.ts
        - 📄 breakpointClient.ts
        - 📄 config.ts
        - 📄 copyFileContents.ts
        - 📄 debouncedBatcher.ts
        - 📄 journal.ts
        - 📄 jsonl.ts
        - 📄 processCatalog.ts
        - 📄 promptBuilder.ts
        - 📄 run.ts
        - 📄 runDetailsSnapshot.ts
        - 📄 runDiscovery.ts
        - 📄 runFileChanges.ts
        - 📄 runId.ts
        - 📄 runPolling.ts
        - 📄 runPresentation.ts
        - 📄 runStatus.ts
        - 📄 sdkDispatch.ts
        - 📄 stateJson.ts
        - 📄 stdinControls.ts
        - 📄 terminalSanitize.ts
        - 📄 textLines.ts
        - 📄 textTailer.ts
        - 📄 workSummaryTailSession.ts
      - 📁 extension
        - 📄 keyFilesModel.ts
        - 📄 promptBuilderBridge.ts
        - 📄 promptBuilderView.ts
        - 📄 runDetailsView.ts
        - 📄 runFileWatchers.ts
        - 📄 runLogsView.ts
        - 📄 runTreeView.ts
      - 📄 extension.ts
      - 📁 test
        - 📄 runTest.ts
        - 📁 suite
          - 📄 constants.ts
          - 📄 dispatch.test.ts
          - 📄 extension.test.ts
          - 📄 index.ts
          - 📄 promptBuilderUi.test.ts
          - 📄 resume.test.ts
      - 📁 unit
        - 📄 awaitingInput.test.ts
        - 📄 copyFileContents.test.ts
        - 📄 debouncedBatcher.test.ts
        - 📄 journal.test.ts
        - 📄 jsonl.test.ts
        - 📄 keyFilesModel.test.ts
        - 📄 manifest.test.ts
        - 📄 processCatalog.test.ts
        - 📄 promptBuilder.test.ts
        - 📄 promptBuilderBridgePreview.test.ts
        - 📄 promptBuilderViewPreviewButton.test.ts
        - 📄 runDetailsSnapshot.test.ts
        - 📄 runDetailsViewWorkSummaryPreviewContract.test.ts
        - 📄 runDiscovery.test.ts
        - 📄 runFileChanges.test.ts
        - 📄 runPolling.test.ts
        - 📄 runPresentation.test.ts
        - 📄 sourceSanity.test.ts
        - 📄 stateJson.test.ts
        - 📄 terminalSanitize.test.ts
        - 📄 textTailer.test.ts
        - 📄 workSummaryTailSession.test.ts
- 📁 plugins
  - 📁 babysitter
    - 📁 skills
      - 📁 babysit
        - 📁 process
          - 📁 gsd
            - 📄 audit-milestone.js
            - 📄 discuss-phase.js
            - 📄 execute-phase.js
            - 📄 iterative-convergence.js
            - 📄 map-codebase.js
            - 📄 new-project.js
            - 📄 plan-phase.js
            - 📄 verify-work.js
          - 📁 methodologies
            - 📄 adversarial-spec-debates.js
            - 📄 agile.js
            - 📁 atdd-tdd
              - 📄 atdd-tdd.js
            - 📄 base44.js
            - 📁 bdd-specification-by-example
              - 📄 bdd-process.js
            - 📄 bottom-up.js
            - 📄 build-realtime-remediation.js
            - 📁 cleanroom
              - 📄 cleanroom.js
            - 📄 consensus-and-voting-mechanisms.js
            - 📄 devin.js
            - 📁 domain-driven-design
              - 📄 domain-driven-design.js
            - 📁 double-diamond
              - 📄 double-diamond.js
            - 📁 event-storming
              - 📄 event-storming.js
            - 📄 evolutionary.js
            - 📁 example-mapping
              - 📄 example-mapping.js
            - 📁 extreme-programming
              - 📄 xp-process.js
            - 📁 feature-driven-development
              - 📄 feature-driven-development.js
            - 📄 graph-of-thoughts.js
            - 📁 hypothesis-driven-development
              - 📄 hypothesis-driven-development.js
            - 📁 impact-mapping
              - 📄 impact-mapping.js
            - 📁 jobs-to-be-done
              - 📄 jobs-to-be-done.js
            - 📁 kanban
              - 📄 kanban.js
            - 📄 plan-and-execute.js
            - 📄 ralph.js
            - 📁 rup
              - 📄 rup.js
            - 📁 scrum
              - 📄 scrum.js
            - 📄 self-assessment.js
            - 📁 shape-up
              - 📄 shape-up.js
            - 📄 spec-driven-development.js
            - 📄 spec-kit-brownfield.js
            - 📄 spec-kit-constitution.js
            - 📄 spec-kit-quality-checklist.js
            - 📁 spiral-model
              - 📄 spiral-model.js
            - 📄 state-machine-orchestration.js
            - 📄 top-down.js
            - 📁 v-model
              - 📄 v-model.js
            - 📁 waterfall
              - 📄 waterfall.js
          - 📄 tdd-quality-convergence.js
- 📁 scripts
  - 📄 bump-version.mjs
  - 📄 check-package-metadata.cjs
  - 📄 release-notes.mjs
  - 📄 update-vscodeignore-deps.mjs
- 📄 test-dispatcher.js
- 📄 test-project-root.js
- 📁 video
  - 📄 remotion.config.ts
  - 📁 src
    - 📄 Root.tsx
    - 📄 Video.tsx
    - 📁 components
      - 📄 ApprovalModal.tsx
      - 📄 Logo.tsx
      - 📄 NeonText.tsx
      - 📄 Pipeline.tsx
      - 📄 ProgressBar.tsx
      - 📄 Terminal.tsx
      - 📄 index.ts
    - 📄 index.ts
    - 📁 scenes
      - 📄 Breakpoints.tsx
      - 📄 CTA.tsx
      - 📄 CompoundingError.tsx
      - 📄 Hook.tsx
      - 📄 QualityConvergence.tsx
      - 📄 Resumability.tsx
      - 📄 index.ts
    - 📁 styles
      - 📄 theme.ts


### File: `packages/breakpoints/extensions/index.js`

- Size: 1312 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 54 lines (0 code)

**Signatures:**

```javascript
// Constants
const require
const telegram
const registry

// Functions
function listExtensions(())
function loadEnabledConfigs((db))

// Constants
const rows
const configs

// Functions
function dispatch((event, db, breakpoint))

// Constants
const configs
const extension

// Functions
function poll((db))

// Constants
const configs
const extension
```

### File: `packages/breakpoints/web/app.js`

- Size: 16867 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 25 functions
- 514 lines (0 code)

**Signatures:**

```javascript
// Constants
const tokenInput
const saveTokenBtn
const statusSelect
const tagInput
const agentInput
const refreshBtn
const listEl
const listMessage
const autoRefreshSelect
const notifySelect
const notifyStatus
const statWaiting
const statReleased
const statExpired
const statCancelled
const detailEmpty
const detailView
const detailTitle
const detailStatus
const detailMeta
const detailPayload
const feedbackList
const feedbackText
const feedbackAuthor
const sendFeedbackBtn
const releaseBtn
const feedbackMessage
const contextList
const contextEmpty
const contextMeta
const contextRender
const contextCode
const contextCodeInner
const extensionsMessage
const extensionsList
const STORAGE_KEY
const STORAGE_REFRESH_KEY
const STORAGE_NOTIFY_KEY
const currentId
const currentStatus
const contextFiles
const refreshTimer
const lastWaitingIds

// Functions
function getToken(())
function setToken((value))
function getAutoRefresh(())
function setAutoRefresh((value))
function getNotifySetting(())
function setNotifySetting((value))
function authHeaders(())

// Constants
const token

// Functions
function apiBase(())

// Constants
const host

// Functions
function setMessage((text))
function updateNotifyStatus(())
function fetchJson((url, options = {}))

// Constants
const target
const res
const body

// Functions
function setStats((items))

// Constants
const counts

// Functions
function statusClass((status))
function getContextFiles((payload))

// Constants
const context

// Functions
function renderContextList(())

// Constants
const li

// Functions
function loadContextFile((file))

// Constants
const res

// Functions
function renderList((items))

// Constants
const li

// Functions
function renderDetail((data))

// Constants
const li

// Functions
function loadList(())

// Constants
const params
const data
const items

// Functions
function maskValue((value))
function loadExtensions(())

// Constants
const data
const items
const card
const isEnabled
const tokenValue
const button
const action
const tokenInput
const userInput
const token
const username
const config
const button

// Functions
function loadDetail((id))

// Constants
const data

// Functions
function sendFeedback((release))

// Constants
const comment
const author

// Functions
function updateWaitingNotifications((items))

// Constants
const waiting
const waitingIds
const newItems

// Functions
function updateAutoRefresh(())

// Constants
const value
const intervalMs
const value
```

### File: `packages/catalog/src/components/catalog/DetailView/index.ts`

- Size: 210 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/catalog/EntityCard/index.ts`

- Size: 257 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/catalog/index.ts`

- Size: 1380 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/common/index.ts`

- Size: 772 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/dashboard/index.ts`

- Size: 749 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/decorations/index.ts`

- Size: 194 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/layout/index.ts`

- Size: 489 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/components/markdown/index.ts`

- Size: 1019 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const MARKDOWN_CSS_PATH
```

### File: `packages/catalog/src/components/ui/index.ts`

- Size: 317 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/hooks/index.ts`

- Size: 137 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/lib/api/index.ts`

- Size: 104 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/lib/db/index.ts`

- Size: 4002 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 162 lines (0 code)

**Signatures:**

```typescript
// Functions
function initializeCatalog() : {
  db: import('./client').CatalogDatabase;
  queries: import('./queries').CatalogQueries;
}
function initializeCatalog() : {
  db: import('./client').CatalogDatabase;
  queries: import('./queries').CatalogQueries;
}

// Constants
const require
const require
const db
const queries

// Functions
function getCatalogSummary() : {
  version: number;
  stats: import('./types').DatabaseStats;
  schema: ReturnType<typeof import('./schema').getSchemaInfo>;
}
function getCatalogSummary() : {
  version: number;
  stats: import('./types').DatabaseStats;
  schema: ReturnType<typeof import('./schema').getSchemaInfo>;
}

// Constants
const require
const require
const db
const stats
const version
const schema
```

### File: `packages/catalog/src/lib/parsers/index.ts`

- Size: 12918 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions, 2 interfaces
- 477 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface UnifiedParserOptions
interface UnifiedParserOptions
interface UnifiedParserResult
interface UnifiedParserResult

// Functions
function parseProcessLibrary(
  options: UnifiedParserOptions
) : Promise<ParseResult<UnifiedParserResult>>
function parseProcessLibrary(
  options: UnifiedParserOptions
) : Promise<ParseResult<UnifiedParserResult>>

// Constants
const options
const errors : Array<{ file: string; error: string }>
const warnings : Array<{ file: string; warning: string }>
const dirResult
const directoryStructure
const agentFiles : string[]
const skillFiles : string[]
const processFiles : string[]
const totalItems
const currentItem
const agents : ParsedAgent[]
const result
const skills : ParsedSkill[]
const result
const processes : ParsedProcess[]
const result
const catalogEntries : CatalogEntry[]
const catalog : ProcessCatalog
const errorMessage

// Functions
function createNodeFileSystem(
  nodeFs: {
    readdir: (path: string) => Promise<string[]>;
    stat: (path: string) => Promise<{ isDirectory(): boolean }>;
    readFile: (path: string, encoding: string) => Promise<string>;
    access?: (path: string) => Promise<void>;
  },
  _nodePath?: { join: (...paths: string[]) => string }
) : FileSystem
function createNodeFileSystem(
  nodeFs: {
    readdir: (path: string) => Promise<string[]>;
    stat: (path: string) => Promise<{ isDirectory(): boolean }>;
    readFile: (path: string, encoding: string) => Promise<string>;
    access?: (path: string) => Promise<void>;
  },
  _nodePath?: { join: (...paths: string[]) => string }
) : FileSystem
function quickParse(
  filePath: string,
  content: string
) : ParseResult<ParsedAgent | ParsedSkill | ParsedProcess>
function quickParse(
  filePath: string,
  content: string
) : ParseResult<ParsedAgent | ParsedSkill | ParsedProcess>

// Constants
const normalized
```

### File: `packages/catalog/src/types/index.ts`

- Size: 1156 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 interfaces
- 66 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ProcessDefinition
interface ProcessDefinition
interface Agent
interface Agent
interface Skill
interface Skill
interface SkillInput
interface SkillInput
interface SkillOutput
interface SkillOutput
interface CatalogItem
interface CatalogItem
interface SearchResult
interface SearchResult
interface FilterOptions
interface FilterOptions
```

### File: `packages/sdk/src/cli/main.ts`

- Size: 52902 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 75 functions, 3 interfaces, 3 types
- 1552 lines (0 code)

**Signatures:**

```typescript
// Constants
const USAGE

// Traits/Interfaces
interface ParsedArgs
interface ActionSummary
interface TaskListEntry

// Constants
const LARGE_RESULT_PREVIEW_LIMIT

// Functions
function parseArgs(argv: string[]) : ParsedArgs

// Constants
const initialCommand
const parsed : ParsedArgs
const positionals : string[]
const i
const arg
const raw
const raw
const raw
const normalized

// Functions
function resolveRunDir(baseDir: string, runDirArg?: string) : string
function expectFlagValue(args: string[], index: number, flag: string) : string

// Constants
const value

// Functions
function parsePositiveInteger(raw: string, flag: string) : number

// Constants
const parsed

// Functions
function summarizeActions(actions: EffectAction[]) : ActionSummary[]
function _logPendingActions(
  actions: EffectAction[],
  options: { command?: string; includeHeader?: boolean; metadataParts?: string[] } = {}
) : ActionSummary[]

// Constants
const summaries
const headerParts
const label

// Functions
function countActionsByKind(actions: EffectAction[]) : Record<string, number>

// Constants
const counts

// Functions
function _enrichIterationMetadata(
  metadata: IterationMetadata | undefined,
  pendingActions?: EffectAction[]
) : IterationMetadata | undefined
function _logSleepHints(command: string, actions: EffectAction[])

// Constants
const sleepMs
const iso
const label
const pendingInfo

// Functions
function formatResolvedEntrypoint(importPath: string, exportName?: string)
function formatVerboseValue(value: unknown) : string
function allowSecretLogs(parsed: ParsedArgs) : boolean

// Constants
const raw
const normalized

// Functions
function logVerbose(command: string, parsed: ParsedArgs, details: Record<string, unknown>)

// Constants
const formatted

// Functions
function isJsonRecord(value: unknown)
function toRunRelativePosix(runDir: string, absolutePath?: string) : string | undefined
function normalizeArtifactRef(runDir: string, ref?: string | null) : string | null

// Constants
const absolute
const relative

// Functions
function resolveArtifactAbsolutePath(runDir: string, ref?: string | null) : string | null

// Constants
const normalized
const absoluteRunDir
const candidates

// Type Aliases
type ArtifactCandidate

// Functions
function collectArtifactCandidates(runDir: string, ref: string) : ArtifactCandidate[]

// Constants
const seen
const pushCandidate
const normalizedAbs
const relative
const outsideRun

// Functions
function defaultResultRef(effectId: string) : string
function formatEntrypointSpecifier(entrypoint: { importPath: string; exportName: string }) : string
function parseEntrypointSpecifier(specifier: string) : { importPath: string; exportName?: string }

// Constants
const hashIndex
const importPath
const exportName

// Type Aliases
type ModuleExports

// Constants
const dynamicImportModule : (specifier: string) => Promise<ModuleExports>

// Functions
function listModuleExports(mod: ModuleExports) : string

// Constants
const keys

// Functions
function validateProcessEntrypoint(importPath: string, exportName?: string) : Promise<void>

// Constants
const resolvedPath
const moduleUrl
const mod : ModuleExports
const resolvedExportName
const candidate
const available

// Functions
function readInputsFile(filePath: string) : Promise<unknown>

// Constants
const absolute
const contents : string

// Functions
function handleRunCreate(parsed: ParsedArgs) : Promise<number>

// Constants
const entrypoint
const runsDir
const absoluteImportPath
const resolvedEntry
const inputs : unknown
const summary
const parts
const result
const entrySpec

// Type Aliases
type RunLifecycleState

// Functions
function handleRunStatus(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const metadata
const journal
const index
const pendingRecords
const pendingByKind
const pendingTotal
const stateSnapshot
const mergedMetadata
const formattedMetadata
const lastEvent
const lastLifecycleEvent
const state
const lastSummary
const completionSecret
const suffix
const completionSecret
const secretSuffix

// Functions
function handleRunIterate(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const result
const countInfo
const actionInfo

// Functions
function handleRunEvents(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const stateSnapshot
const journal
const filterType
const filtered
const orderedBase
const ordered
const limited
const metadata
const formattedMetadata
const headerParts
const metadataSuffix

// Functions
function handleRunRebuildState(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const plan
const snapshot
const metadata : IterationMetadata
const formatted
const suffix

// Functions
function handleRunRepairJournal(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const journalDir
const files
const rawEvents : Array<{ filename: string; payload: { type?: unknown; recordedAt?: unknown; data?: unknown } }>
const fullPath
const payload
const seenInvocation
const keptEffectIds
const droppedEffectIds
const kept : Array<{ type: string; recordedAt?: string; data: JsonRecord }>
const droppedRequested
const droppedResolved
const type
const recordedAt
const data
const invocationKey
const effectId
const effectId
const summary
const stamp
const repairedDir
const i
const seq
const ulid
const filename
const eventPayload : JsonRecord
const contents
const checksum
const withChecksum
const backupDir

// Functions
function handleTaskPost(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const secretLogsAllowed
const index
const record
const nowIso
const startedAt
const finishedAt
const resolveMaybeRunRelative
const readJsonFile : Promise<unknown>
const raw
const trimmed
const absolute
const raw
const trimmed
const readTextFile : Promise<string | undefined>
const absolute
const metadataRaw
const metadata : JsonRecord | undefined
const stdout
const stderr
const value : unknown
const errorPayload : unknown
const invocationKey
const plan
const committed
const stdoutRef
const stderrRef
const resultRef
const parts

// Functions
function handleTaskList(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const index
const rawRecords
const records
const entries
const scope
const label

// Functions
function handleTaskShow(parsed: ParsedArgs) : Promise<number>

// Constants
const runDir
const secretLogsAllowed
const index
const record
const taskDef
const preview
const entry
const inlineResult
const largeResultRef

// Functions
function toTaskListEntry(record: EffectRecord, runDir: string) : TaskListEntry
function buildEffectIndexSafe(runDir: string, command: string, events?: JournalEvent[])
function readRunMetadataSafe(runDir: string, command: string) : Promise<RunMetadata | null>
function loadJournalSafe(runDir: string, command: string) : Promise<JournalEvent[] | null>
function readStateCacheSafe(runDir: string, command: string) : Promise<StateCacheSnapshot | null>

// Constants
const snapshot
const RUN_LIFECYCLE_TYPES : ReadonlySet<JournalEvent["type"]>

// Functions
function findLastLifecycleEvent(events: JournalEvent[]) : JournalEvent | undefined

// Constants
const i
const event

// Functions
function countPendingByKind(records: EffectRecord[]) : Record<string, number>

// Constants
const counts
const key

// Functions
function loadTaskResultPreview(
  runDir: string,
  effectId: string,
  record: EffectRecord
) : Promise<{ result?: StoredTaskResult; large: boolean }>

// Constants
const absolutePath
const stats
const err
const data

// Functions
function readStdinUtf8() : Promise<string>

// Constants
const chunks : Buffer[]

// Functions
function deriveRunState(
  lastLifecycleEventType: JournalEvent["type"] | undefined,
  pendingTotal: number
) : RunLifecycleState
function formatLastEventSummary(event?: JournalEvent) : string
function mergeMetadataSources(
  metadata: IterationMetadata | undefined,
  options: { snapshot?: StateCacheSnapshot | null; pendingByKind?: Record<string, number> }
) : IterationMetadata | undefined

// Constants
const snapshot
const hasPendingOverride
const snapshotHasInfo
const next : IterationMetadata

// Functions
function formatIterationMetadata(
  metadata?: IterationMetadata
) : { textParts: string[]; jsonMetadata?: IterationMetadata }

// Constants
const textParts : string[]
const seq
const reasonSuffix
const pendingEntries
const pendingTotal

// Functions
function serializeJournalEvent(event: JournalEvent, runDir: string)

// Constants
const data

// Functions
function ensureIterationMetadata(data: Record<string, unknown>) : Record<string, unknown>

// Constants
const iteration
const iterationRecord
const metadata

// Functions
function formatEventLine(event: JournalEvent) : string
function formatSeq(seq: number) : string
function readCliVersion() : Promise<string>

// Constants
const packagePath
const raw
const parsed

// Functions
function createBabysitterCli()
function createBabysitterCli()

// Constants
const parsed
```

### File: `packages/sdk/src/hooks/index.ts`

- Size: 801 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 41 lines (0 code)

**Signatures:**

```typescript
// Functions
function createHookPayload(
  payload: T
) : T & { timestamp: string }
function createHookPayload(
  payload: T
) : T & { timestamp: string }
```

### File: `packages/sdk/src/index.ts`

- Size: 225 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/runtime/index.ts`

- Size: 1136 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/runtime/intrinsics/index.ts`

- Size: 344 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/runtime/invocation/index.ts`

- Size: 144 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/runtime/replay/index.ts`

- Size: 569 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/storage/index.ts`

- Size: 526 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/tasks/index.ts`

- Size: 193 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/src/tasks/kinds/index.ts`

- Size: 14677 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 36 functions, 1 types
- 466 lines (0 code)

**Signatures:**

```typescript
// Constants
const DEFAULT_NODE_TIMEOUT_MS
const DEFAULT_BREAKPOINT_LABEL
const DEFAULT_ORCHESTRATOR_LABEL
const DEFAULT_SLEEP_TASK_ID
const REDACTION_KEYWORDS

// Type Aliases
type EnvInput

// Functions
function nodeTask(
  id: string,
  options: NodeTaskDefinitionOptions<TArgs>
) : DefinedTask<TArgs, TResult>
function nodeTask(
  id: string,
  options: NodeTaskDefinitionOptions<TArgs>
) : DefinedTask<TArgs, TResult>

// Constants
const entry
const io
const sanitizeEnv
const metadata
const labels
const nodeOptions

// Functions
function breakpointTask(
  id: string,
  options: BreakpointTaskDefinitionOptions<TArgs> = {}
) : DefinedTask<TArgs, TResult>
function breakpointTask(
  id: string,
  options: BreakpointTaskDefinitionOptions<TArgs> = {}
) : DefinedTask<TArgs, TResult>

// Constants
const title
const labels
const breakpoint
const resolvedTitle

// Functions
function orchestratorTask(
  id: string,
  options: OrchestratorTaskDefinitionOptions<TArgs> = {}
) : DefinedTask<TArgs, TResult>
function orchestratorTask(
  id: string,
  options: OrchestratorTaskDefinitionOptions<TArgs> = {}
) : DefinedTask<TArgs, TResult>

// Constants
const title
const labels
const orchestrator
const mergedMetadata
const resolvedTitle

// Functions
function sleepTask(
  id = DEFAULT_SLEEP_TASK_ID,
  options: SleepTaskDefinitionOptions<TArgs> = {}
) : DefinedTask<TArgs, void>
function sleepTask(
  id = DEFAULT_SLEEP_TASK_ID,
  options: SleepTaskDefinitionOptions<TArgs> = {}
) : DefinedTask<TArgs, void>

// Constants
const title
const iso
const targetEpochMs
const labels
const mergedMetadata
const resolvedTitle

// Functions
function resolveOptionalValue(
  source: TaskValueOrFactory<TArgs, TValue | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<TValue | undefined>

// Constants
const value

// Functions
function resolveRequiredValue(
  source: TaskValueOrFactory<TArgs, string>,
  args: TArgs,
  ctx: TaskBuildContext,
  field: string
) : Promise<string>

// Constants
const value

// Functions
function resolveLabelList(
  source: TaskValueOrFactory<TArgs, string[] | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<string[] | undefined>

// Constants
const values

// Functions
function resolveMetadata(
  source: TaskValueOrFactory<TArgs, JsonRecord | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<JsonRecord | undefined>

// Constants
const value

// Functions
function resolveIoHints(
  source: TaskValueOrFactory<TArgs, TaskIOHints | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<TaskIOHints | undefined>

// Constants
const value

// Functions
function resolveStringArray(
  source: TaskValueOrFactory<TArgs, string[] | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<string[] | undefined>

// Constants
const values

// Functions
function resolveEnv(
  source: TaskValueOrFactory<TArgs, EnvInput | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<EnvInput | undefined>

// Constants
const value

// Functions
function resolveNumber(
  source: TaskValueOrFactory<TArgs, number | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<number | undefined>

// Constants
const value

// Functions
function resolveBoolean(
  source: TaskValueOrFactory<TArgs, boolean | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<boolean | undefined>

// Constants
const value

// Functions
function resolvePayload(
  source: TaskValueOrFactory<TArgs, JsonRecord | undefined> | undefined,
  args: TArgs,
  ctx: TaskBuildContext
) : Promise<JsonRecord | undefined>

// Constants
const value

// Functions
function isFactory(value: TaskValueOrFactory<TArgs, TValue>)
function normalizeLabels(values?: string[]) : string[] | undefined

// Constants
const seen
const normalized : string[]
const trimmed

// Functions
function mergeLabels(ctx: TaskBuildContext, helperLabels?: string[], fallback?: string) : string[] | undefined

// Constants
const combined : string[]
const normalized
const fallbackLabel

// Functions
function buildDefaultNodeIo(ctx: TaskBuildContext) : TaskIOHints
function applyIoOverrides(base: TaskIOHints, override?: TaskIOHints) : TaskIOHints
function sanitizeEnv(input?: EnvInput) : { env?: Record<string, string>; redacted: string[] }

// Constants
const env : Record<string, string>
const redacted : string[]
const key
const value

// Functions
function shouldRedactEnvKey(key: string) : boolean

// Constants
const upper

// Functions
function appendRedactedEnvKeys(metadata: JsonRecord | undefined, keys: string[]) : JsonRecord | undefined

// Constants
const result
const existingValue
const existing : string[]
const merged

// Functions
function ensureMetadata(metadata?: JsonRecord) : JsonRecord
function buildNodeOptions(
  entry: string,
  args: string[] | undefined,
  env: Record<string, string> | undefined,
  cwd: string | undefined,
  timeoutOverride: number | undefined
) : NodeTaskOptions

// Constants
const nodeOptions : NodeTaskOptions

// Functions
function buildBreakpointOptions(payload: unknown, confirmationRequired?: boolean) : BreakpointTaskOptions

// Constants
const breakpoint : BreakpointTaskOptions

// Functions
function buildOrchestratorOptions(payload?: JsonRecord, resumeCommand?: string) : OrchestratorTaskOptions

// Constants
const orchestrator : OrchestratorTaskOptions

// Functions
function pickJsonRecord(value: unknown) : JsonRecord | undefined
function isJsonRecord(value: unknown)
```

### File: `packages/sdk/src/test-fixtures/kinds/index.ts`

- Size: 1212 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const sleepIso
const nodeKindFixtures
const breakpointKindFixtures
const orchestratorKindFixtures
const sleepKindFixtures
```

### File: `packages/sdk/src/testing/index.ts`

- Size: 198 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/test-fixtures/kinds/index.js`

- Size: 1506 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const sleepIso
```

### File: `packages/sdk/test-fixtures/kinds/index.ts`

- Size: 1212 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const sleepIso
const nodeKindFixtures
const breakpointKindFixtures
const orchestratorKindFixtures
const sleepKindFixtures
```

### File: `video/src/components/index.ts`

- Size: 277 bytes
- Modified: 2026-02-15 10:31:50 UTC


### File: `video/src/index.ts`

- Size: 93 bytes
- Modified: 2026-02-15 10:31:50 UTC


### File: `video/src/scenes/index.ts`

- Size: 266 bytes
- Modified: 2026-02-15 10:31:50 UTC


### File: `packages/babysitter/bin/babysitter.js`

- Size: 264 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const require
```

### File: `packages/breakpoints/api/db.js`

- Size: 2254 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions
- 93 lines (0 code)

**Signatures:**

```javascript
// Constants
const fs
const path
const os
const sqlite3

// Functions
function defaultDbPath(())

// Constants
const home

// Functions
function expandHome((value))
function resolveDbPath((value))

// Constants
const expanded
const normalized
const DB_PATH

// Functions
function openDb(())
function run((db, sql, params = []))
function get((db, sql, params = []))
function all((db, sql, params = []))
function initDb((db))

// Constants
const schemaPath
const schema
const statements
```

### File: `packages/breakpoints/api/routes.js`

- Size: 13762 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions
- 475 lines (0 code)

**Signatures:**

```javascript
// Constants
const express
const crypto
const fs
const path
const require
const extensions
const extensions
const require
const router

// Functions
function nowIso(())
function requireToken((expected))

// Constants
const header
const token

// Functions
function parseTags((raw))
function serializeTags((tags))
function deserializeTags((value))

// Constants
const parsed

// Functions
function repoRoot(())
function normalizeContextFiles((payload))

// Constants
const context

// Functions
function allowlistedExtension((filePath))

// Constants
const ext

// Functions
function resolveContextPath((requestedPath))

// Constants
const root
const rootResolved
const resolved
const rootWithSep

// Functions
function inferFormat((filePath, hint))

// Constants
const ext

// Functions
function inferLanguage((filePath, hint))

// Constants
const ext
const map

// Functions
function withDb((fn))

// Constants
const db
const requireAgent
const requireHuman
const id
const createdAt
const tagList
const payloadJson
const tagsJson
const runAt
const jobPayload
const req
const limitNum
const offsetNum
const clauses
const params
const where
const rows
const items
const req
const row
const feedback
const req
const row
const req
const requestedPath
const row
const payload
const files
const match
const resolved
const stat
const content
const format
const language
const req
const actor
const ts
const updated
const row
const req
const ts
const result
const row
const rows
const known
const byName
const items
const existing
const req
const row
const req
const actor
const ts
const updated
const row
```

### File: `packages/breakpoints/api/server.js`

- Size: 848 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const path
const express
const routes
const app
const port
```

### File: `packages/breakpoints/bin/breakpoints.js`

- Size: 5805 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 220 lines (0 code)

**Signatures:**

```javascript
// Constants
const require
const path
const fs

// Functions
function usage(())
function runCommand((command, args, options = {}))

// Constants
const proc

// Functions
function apiBase(())
function httpJson((method, url, body))

// Constants
const headers
const res
const err

// Functions
function parseFlags((argv))

// Constants
const flags
const i
const arg
const key
const value
const existing

// Functions
function parseFiles((flagValue))

// Constants
const values

// Functions
function breakpointCreate((flags))

// Constants
const question
const files
const payload
const body
const result

// Functions
function breakpointStatus((id))

// Constants
const result

// Functions
function breakpointShow((id))

// Constants
const result

// Functions
function breakpointWait((id, intervalSeconds))

// Constants
const interval
const status
const res
const details

// Functions
function runSystem(())

// Constants
const repoRoot
const runner
const env
const args
const cmd
const packagePath
const pkg
const flags
const subcmd
const id
```

### File: `packages/breakpoints/extensions/config.js`

- Size: 1158 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 47 lines (0 code)

**Signatures:**

```javascript
// Constants
const require

// Functions
function listConfigs((db))
function getConfig((db, name))
function upsertConfig((db, name, enabled, config))

// Constants
const now
const configJson

// Functions
function parseConfig((row))

// Constants
const config
```

### File: `packages/breakpoints/extensions/telegram.js`

- Size: 26628 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 22 functions
- 896 lines (0 code)

**Signatures:**

```javascript
// Constants
const https
const crypto
const require
const require

// Functions
function apiBase((token))
function callTelegram((url, options = {}))

// Constants
const options
const attempt
const makeRequest
const req
const data
const handleError
const isRetryable
const delay

// Functions
function sendMessage((token, chatId, text, parseMode))

// Constants
const mode
const url

// Functions
function sendDocument((token, chatId, filename, content, options = {}))

// Constants
const options
const boundary
const payload
const url
const attempt
const makeRequest
const req
const data
const handleError
const isRetryable
const delay

// Functions
function allowlistedExtension((filePath))

// Constants
const ext

// Functions
function resolveRepoPath((requestedPath))

// Constants
const path
const root
const resolved
const rootWithSep

// Functions
function readContextFile((db, breakpointId, requestedPath))

// Constants
const row
const payload
const files
const match
const abs
const fs

// Functions
function listContextFiles((db, breakpointId))

// Constants
const row
const payload

// Functions
function validateToken((token))

// Constants
const url
const response

// Functions
function resolveTelegramUser((token, username))

// Constants
const validation
const url
const response
const updates
const lower
const match
const from
const sessionCache

// Functions
function cacheKey((token, username))
function rememberUser((token, username, chatId, userId))
function lookupUser((token, username))
function readStoredUser((config))
function persistUser((db, config, chatId, userId))
function onBreakpointCreated((db, breakpoint, config))

// Constants
const summary
const question
const files
const fileLines
const format
const label
const text
const cached
const stored
const target

// Functions
function onBreakpointReleased((db, breakpoint, config))

// Constants
const summary
const text
const cached
const stored
const target

// Functions
function escapeMarkdownV2((text))
function escapeCodeBlock((text))
function inferLanguageFromPath((filePath))

// Constants
const ext
const map

// Functions
function buildRawMessage((file, originalPath))

// Constants
const language
const header
const maxLen
const body
const escaped

// Functions
function poll((db, config))

// Constants
const offset
const url
const response
const updates
const maxUpdateId
const message
const username
const replyText
const replyMatch
const allowByReply
const normalized
const usernameLower
const trimmed
const listMatch
const require
const waitingBreakpoints
const messages
const i
const bp
const title
const runId
const age
const ageStr
const payload
const question
const require
const waitingBreakpoints
const messages
const i
const bp
const title
const runId
const age
const ageStr
const payload
const question
const previewMatch
const listNumber
const require
const waitingBreakpoints
const bp
const payload
const title
const question
const files
const fileLines
const format
const label
const age
const ageStr
const previewText
const fileMatch
const mode
const requestedArg
const targetId
const uuidMatch
const waiting
const file
const indexMatch
const files
const idx
const pathValue
const text
const result
const id
const comment
const uuidMatch
const waiting
const row
const ts
const ack
const ackChatId
const ackResult
```

### File: `packages/breakpoints/scripts/dev-runner.js`

- Size: 1042 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 41 lines (0 code)

**Signatures:**

```javascript
// Constants
const require
const path
const repoRoot
const defaultEnv

// Functions
function run((label, command, args))

// Constants
const proc
```

### File: `packages/breakpoints/scripts/init-db.js`

- Size: 401 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 20 lines (0 code)

**Signatures:**

```javascript
// Constants
const require

// Functions
function main(())

// Constants
const db
```

### File: `packages/breakpoints/web/server.js`

- Size: 335 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const path
const express
const app
const port
```

### File: `packages/breakpoints/worker/queue.js`

- Size: 1989 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions
- 85 lines (0 code)

**Signatures:**

```javascript
// Constants
const crypto
const require

// Functions
function claimJobs((db, limit))

// Constants
const now
const jobs

// Functions
function completeJob((db, jobId))

// Constants
const now

// Functions
function failJob((db, jobId, error))

// Constants
const now

// Functions
function expireBreakpoint((db, breakpointId))

// Constants
const now
const row

// Functions
function processJob((db, job))

// Constants
const payload
```

### File: `packages/breakpoints/worker/worker.js`

- Size: 2107 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 71 lines (0 code)

**Signatures:**

```javascript
// Constants
const require
const require
const extensions
const POLL_INTERVAL_MS
const BATCH_SIZE

// Functions
function runOnce(())

// Constants
const db
const jobs

// Functions
function loop(())

// Constants
const consecutiveErrors
const MAX_CONSECUTIVE_ERRORS
const backoffDelay
```

### File: `packages/catalog/eslint.config.mjs`

- Size: 465 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const eslintConfig
```

### File: `packages/catalog/next.config.ts`

- Size: 762 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const nextConfig : NextConfig
```

### File: `packages/catalog/postcss.config.mjs`

- Size: 94 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const config
```

### File: `packages/catalog/process-library-catalog.js`

- Size: 31899 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 861 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const startTime
const artifacts
const projectSetup
const parsersResult
const indexerResult
const apiResult
const baseComponentsResult
const catalogComponentsResult
const markdownResult
const pagesResult
const dashboardResult
const integrationResult
const qualityScore
const iteration
const maxIterations
const buildResult
const scoringResult
const polishResult
const endTime
const projectSetupTask
const createParsersTask
const createIndexerTask
const createApiRoutesTask
const createBaseComponentsTask
const createCatalogComponentsTask
const createMarkdownRendererTask
const createPagesTask
const createDashboardTask
const integrateComponentsTask
const buildCheckTask
const lintCheckTask
const typeCheckTask
const qualityScoringTask
const fixIssuesTask
const finalPolishTask
```

### File: `packages/catalog/scripts/reindex.ts`

- Size: 9974 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions, 1 interfaces
- 284 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface CLIOptions

// Functions
function parseArgs() : CLIOptions

// Constants
const args
const options : CLIOptions

// Functions
function showHelp() : void

// Constants
const spinnerFrames
const spinnerIndex

// Functions
function getSpinner() : string
function formatDuration(ms: number) : string

// Constants
const minutes
const seconds

// Functions
function formatBytes(bytes: number) : string
function createProgressCallback(verbose: boolean) : (progress: IndexProgress) => void

// Constants
const lastPhase
const progress
const percent
const fileName
const percent
const bar

// Functions
function main() : Promise<void>

// Constants
const options
const dbPath
const exists
const baseDir
const indexer
const startTime
const result
const duration
const maxErrors
const i
const error
const db
const stats
```

### File: `packages/catalog/src/app/agents/[slug]/page.tsx`

- Size: 3699 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 1 interfaces
- 132 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PageProps

// Functions
function getAgent(slug: string) : Promise<AgentDetailType | null>

// Constants
const baseUrl
const res
const json

// Functions
function getRelatedAgents(
  domainName: string | null,
  specializationName: string | null,
  currentId: number
) : Promise<AgentListItem[]>

// Constants
const baseUrl
const params
const res
const json
const params
const agent
const relatedAgents
const breadcrumbItems : BreadcrumbItem[]
const params
const agent
```

### File: `packages/catalog/src/app/agents/loading.tsx`

- Size: 1496 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/app/agents/page.tsx`

- Size: 7938 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions
- 225 lines (0 code)

**Signatures:**

```typescript
// Constants
const router
const searchParams
const agents
const domains
const expertiseOptions
const total
const isLoading
const initialDomain
const initialExpertise
const initialPage
const initialSearch
const searchQuery
const filterDomain
const filterExpertise
const currentPage
const itemsPerPage
const filterExpertiseKey
const fetchReferenceData
const domainsRes
const json
const agentsRes
const json
const allExpertise
const fetchAgents
const params
const res
const json
const updateUrl
const params
const search
const url
const handleSearch
const handleFilterChange
const newDomain
const newExpertise
const handlePageChange
const filteredAgents
const domainNames
```

### File: `packages/catalog/src/app/api/agents/[slug]/route.ts`

- Size: 2710 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 108 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface RouteContext

// Functions
function GET(
  _request: NextRequest,
  context: RouteContext
)
function GET(
  _request: NextRequest,
  context: RouteContext
)

// Constants
const params
const validation
const slug
const db
const rawDb
const sql
const row
const agent : AgentDetail
```

### File: `packages/catalog/src/app/api/agents/route.ts`

- Size: 3981 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 141 lines (0 code)

**Signatures:**

```typescript
// Constants
const SORT_FIELDS : Record<string, string>

// Functions
function GET(request: NextRequest)
function GET(request: NextRequest)

// Constants
const searchParams
const parseListQueryParams
const specialization
const domain
const expertise
const db
const rawDb
const sql
const conditions : string[]
const params : unknown[]
const sortField
const countSql
const countParams
const countResult
const total
const rows
const agents : AgentListItem[]
```

### File: `packages/catalog/src/app/api/analytics/route.ts`

- Size: 3565 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 116 lines (0 code)

**Signatures:**

```typescript
// Functions
function GET(_request: NextRequest)
function GET(_request: NextRequest)

// Constants
const db
const rawDb
const stats
const byDomainSql
const byDomainRows
const byDomain : EntityDistribution[]
const byCategorySql
const byCategoryRows
const byCategory : EntityDistribution[]
const byType : EntityDistribution[]
const recentActivitySql
const recentRows
const recentActivity : RecentActivityItem[]
const analytics : AnalyticsResponse
```

### File: `packages/catalog/src/app/api/domains/[slug]/route.ts`

- Size: 3390 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 124 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface RouteContext

// Functions
function GET(
  _request: NextRequest,
  context: RouteContext
)
function GET(
  _request: NextRequest,
  context: RouteContext
)

// Constants
const params
const validation
const slug
const db
const rawDb
const domainSql
const domainRow
const specsSql
const specRows
const specializations : SpecializationSummary[]
const domain : DomainDetail
```

### File: `packages/catalog/src/app/api/domains/route.ts`

- Size: 3020 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 105 lines (0 code)

**Signatures:**

```typescript
// Constants
const SORT_FIELDS : Record<string, string>

// Functions
function GET(request: NextRequest)
function GET(request: NextRequest)

// Constants
const searchParams
const parseListQueryParams
const category
const db
const rawDb
const sql
const params : unknown[]
const sortField
const countSql
const countResult
const total
const rows
const domains : DomainListItem[]
```

### File: `packages/catalog/src/app/api/processes/[id]/route.ts`

- Size: 1883 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 71 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface RouteContext

// Functions
function GET(
  _request: NextRequest,
  context: RouteContext
)
function GET(
  _request: NextRequest,
  context: RouteContext
)

// Constants
const params
const validation
const processId
const db
const queries
const row
const inputs
const outputs
const tasks
const frontmatter
const process : ProcessDetail
```

### File: `packages/catalog/src/app/api/processes/route.ts`

- Size: 2126 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 77 lines (0 code)

**Signatures:**

```typescript
// Constants
const SORT_FIELDS : Record<string, string>

// Functions
function GET(request: NextRequest)
function GET(request: NextRequest)

// Constants
const searchParams
const parseListQueryParams
const category
const db
const builder
const sortField
const total
const rows
const processes : ProcessListItem[]
const tasks
```

### File: `packages/catalog/src/app/api/reindex/route.ts`

- Size: 1917 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 70 lines (0 code)

**Signatures:**

```typescript
// Functions
function POST(request: NextRequest)
function POST(request: NextRequest)

// Constants
const force
const body
const baseDir
const result
const response : ReindexResponse

// Functions
function GET(request: NextRequest)
function GET(request: NextRequest)

// Constants
const searchParams
const force
const baseDir
const result
const response : ReindexResponse
```

### File: `packages/catalog/src/app/api/search/route.ts`

- Size: 2243 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 73 lines (0 code)

**Signatures:**

```typescript
// Functions
function GET(request: NextRequest)
function GET(request: NextRequest)

// Constants
const searchParams
const qResult
const query
const parseListQueryParams
const typeParam
const types : CatalogEntryType[]
const validTypes : CatalogEntryType[]
const db
const queries
const allResults
const paginatedResults
const total
const results : SearchResultItem[]
```

### File: `packages/catalog/src/app/api/skills/[slug]/route.ts`

- Size: 2676 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 105 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface RouteContext

// Functions
function GET(
  _request: NextRequest,
  context: RouteContext
)
function GET(
  _request: NextRequest,
  context: RouteContext
)

// Constants
const params
const validation
const slug
const db
const rawDb
const sql
const row
const skill : SkillDetail
```

### File: `packages/catalog/src/app/api/skills/route.ts`

- Size: 3945 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 137 lines (0 code)

**Signatures:**

```typescript
// Constants
const SORT_FIELDS : Record<string, string>

// Functions
function GET(request: NextRequest)
function GET(request: NextRequest)

// Constants
const searchParams
const parseListQueryParams
const specialization
const domain
const category
const db
const rawDb
const sql
const conditions : string[]
const params : unknown[]
const sortField
const countSql
const countParams
const countResult
const total
const rows
const skills : SkillListItem[]
```

### File: `packages/catalog/src/app/domains/[slug]/page.tsx`

- Size: 7461 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 212 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PageProps

// Functions
function getDomain(slug: string) : Promise<DomainDetail | null>

// Constants
const baseUrl
const res
const json
const params
const domain
const params
const domain
```

### File: `packages/catalog/src/app/domains/page.tsx`

- Size: 6344 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 188 lines (0 code)

**Signatures:**

```typescript
// Constants
const domains
const specializations
const isLoading
const fetchData
const domainsRes
const json
const json
const treeData : TreeNode[]
const domainSpecs
const stats
```

### File: `packages/catalog/src/app/layout.tsx`

- Size: 1727 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 65 lines (0 code)

**Signatures:**

```typescript
// Constants
const playfair
const garamond
const cinzelDecorative
const metadata : Metadata

// Functions
function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>)
function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>)
```

### File: `packages/catalog/src/app/page.tsx`

- Size: 10501 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 268 lines (0 code)

**Signatures:**

```typescript
// Functions
function getAnalytics() : Promise<AnalyticsResponse | null>

// Constants
const baseUrl
const res
const json
const analytics
const barChartData : BarChartData[]
const pieChartData : PieChartData[]
const treemapData : TreemapData[]
const quickLinks : QuickLinkItem[]
```

### File: `packages/catalog/src/app/processes/[id]/page.tsx`

- Size: 2546 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 1 interfaces
- 89 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PageProps

// Functions
function getProcess(id: string) : Promise<ProcessDetailType | null>

// Constants
const baseUrl
const res
const json

// Functions
function getRelatedProcesses(category: string | null, currentId: number) : Promise<ProcessListItem[]>

// Constants
const baseUrl
const params
const res
const json

// Functions
function ProcessDetailPage({ params }: PageProps)
function ProcessDetailPage({ params }: PageProps)

// Constants
const params
const process
const relatedProcesses

// Functions
function generateMetadata({ params }: PageProps)
function generateMetadata({ params }: PageProps)

// Constants
const params
const process
```

### File: `packages/catalog/src/app/processes/loading.tsx`

- Size: 1315 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/app/processes/page.tsx`

- Size: 6676 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 194 lines (0 code)

**Signatures:**

```typescript
// Constants
const router
const searchParams
const processes
const total
const isLoading
const categories
const initialCategory
const initialPage
const initialSearch
const searchQuery
const filterCategory
const currentPage
const itemsPerPage
const fetchProcesses
const params
const res
const json
const uniqueCategories
const fetchCategories
const res
const json
const uniqueCategories
const updateUrl
const params
const search
const url
const handleSearch
const handleFilterChange
const newCategory
const handlePageChange
const filteredProcesses
```

### File: `packages/catalog/src/app/search/loading.tsx`

- Size: 1001 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/app/search/page.tsx`

- Size: 48008 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 18 functions, 1 types
- 1130 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type EntityType

// Functions
function GearIcon({ className, size = 16 }: { className?: string; size?: number })
function GlobeIcon({ className, size = 16 }: { className?: string; size?: number })

// Constants
const router
const searchParams
const initialQuery
const initialType
const query
const inputValue
const activeType
const results
const isLoading
const hasSearched
const performSearch
const params
const res
const json
const initialSearchDoneRef
const handleSearch
const handleTypeChange
const updateUrl
const params
const search
const url
const clearSearch
const clearFilters
const groupedResults
const groups : Record<string, SearchResultItem[]>
const type
const typeOrder : EntityType[]
const sortedTypes
const activeFilters : { key: string; label: string }[]
const typeResults
```

### File: `packages/catalog/src/app/skills/[slug]/page.tsx`

- Size: 3675 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 1 interfaces
- 131 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PageProps

// Functions
function getSkill(slug: string) : Promise<SkillDetailType | null>

// Constants
const baseUrl
const res
const json

// Functions
function getRelatedSkills(
  domainName: string | null,
  specializationName: string | null,
  currentId: number
) : Promise<SkillListItem[]>

// Constants
const baseUrl
const params
const res
const json
const params
const skill
const relatedSkills
const breadcrumbItems : BreadcrumbItem[]
const params
const skill
```

### File: `packages/catalog/src/app/skills/loading.tsx`

- Size: 1312 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/catalog/src/app/skills/page.tsx`

- Size: 7044 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 204 lines (0 code)

**Signatures:**

```typescript
// Constants
const router
const searchParams
const skills
const domains
const _specializations
const total
const isLoading
const initialDomain
const initialSpecialization
const initialPage
const initialSearch
const searchQuery
const filterDomain
const filterSpecialization
const currentPage
const itemsPerPage
const fetchReferenceData
const domainsRes
const json
const json
const fetchSkills
const params
const res
const json
const updateUrl
const params
const search
const url
const handleSearch
const handleFilterChange
const newDomain
const handlePageChange
const filteredSkills
const domainNames
```

### File: `packages/catalog/src/components/ErrorBoundary.tsx`

- Size: 9469 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions, 1 classes, 5 interfaces
- 316 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ErrorBoundaryProps
interface ErrorBoundaryProps
interface ErrorBoundaryState
interface ErrorBoundaryState

// Structs/Classes
class ErrorBoundary
class ErrorBoundary

// Traits/Interfaces
interface ErrorFallbackProps
interface ErrorFallbackProps

// Functions
function ErrorFallback({
  error,
  errorInfo,
  onReset,
  showRetry = true,
  title = "Something went wrong",
  description = "An error occurred while rendering this page.",
}: ErrorFallbackProps)
function ErrorFallback({
  error,
  errorInfo,
  onReset,
  showRetry = true,
  title = "Something went wrong",
  description = "An error occurred while rendering this page.",
}: ErrorFallbackProps)

// Constants
const isDev

// Traits/Interfaces
interface SuspenseErrorBoundaryProps

// Functions
function SuspenseErrorBoundary({
  children,
  fallback,
  errorFallback,
}: SuspenseErrorBoundaryProps)
function SuspenseErrorBoundary({
  children,
  fallback,
  errorFallback,
}: SuspenseErrorBoundaryProps)

// Traits/Interfaces
interface AsyncBoundaryProps
interface AsyncBoundaryProps

// Functions
function AsyncBoundary({ children, loading, error }: AsyncBoundaryProps)
function AsyncBoundary({ children, loading, error }: AsyncBoundaryProps)
function DefaultLoadingFallback()
```

### File: `packages/catalog/src/components/catalog/DetailView/AgentDetail.tsx`

- Size: 5915 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 173 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface AgentDetailProps
interface AgentDetailProps
```

### File: `packages/catalog/src/components/catalog/DetailView/ProcessDetail.tsx`

- Size: 9720 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 1 interfaces
- 264 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ProcessDetailProps
interface ProcessDetailProps

// Constants
const expandedTasks
const toggleTask
const next
const expandAllTasks
const collapseAllTasks
```

### File: `packages/catalog/src/components/catalog/DetailView/SkillDetail.tsx`

- Size: 5619 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 166 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SkillDetailProps
interface SkillDetailProps
```

### File: `packages/catalog/src/components/catalog/EntityCard/AgentCard.tsx`

- Size: 5553 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 178 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface AgentCardProps
interface AgentCardProps

// Constants
const isCompact
```

### File: `packages/catalog/src/components/catalog/EntityCard/DomainCard.tsx`

- Size: 5347 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 interfaces
- 168 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface DomainCardProps
interface DomainCardProps

// Constants
const isCompact
const totalEntities
```

### File: `packages/catalog/src/components/catalog/EntityCard/ProcessCard.tsx`

- Size: 4717 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 154 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ProcessCardProps
interface ProcessCardProps

// Constants
const isCompact
const date
const now
const diffInSeconds
```

### File: `packages/catalog/src/components/catalog/EntityCard/SkillCard.tsx`

- Size: 5707 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 179 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SkillCardProps
interface SkillCardProps

// Constants
const isCompact
```

### File: `packages/catalog/src/components/catalog/EntityList.tsx`

- Size: 7192 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces, 1 types
- 229 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type ViewMode

// Traits/Interfaces
interface EntityListProps
interface EntityListProps

// Constants
const gridColsClasses
const total
const totalPages
const getGridClass
const classes
```

### File: `packages/catalog/src/components/catalog/FilterPanel.tsx`

- Size: 11746 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions, 2 interfaces, 1 types
- 328 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type EntityType

// Traits/Interfaces
interface FilterPanelProps
interface FilterPanelProps
interface FilterValues
interface FilterValues

// Constants
const entityTypeOptions : Array<{ value: EntityType; label: string; icon: React.ReactNode }>
const isCollapsed
const handleEntityTypeToggle
const currentTypes
const newTypes
const handleDomainChange
const handleCategoryChange
const handleExpertiseToggle
const currentExpertise
const newExpertise
const handleClearAll
const hasActiveFilters
const activeFilterCount
const isChecked
const isSelected
```

### File: `packages/catalog/src/components/catalog/MetadataDisplay.tsx`

- Size: 7271 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions, 1 interfaces
- 251 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface MetadataDisplayProps
interface MetadataDisplayProps

// Constants
const copied
const viewMode
const handleCopy
const toggleMode
const hasComplexValues
```

### File: `packages/catalog/src/components/catalog/QuickActions.tsx`

- Size: 4654 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 1 interfaces
- 139 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface QuickActionsProps
interface QuickActionsProps

// Constants
const copied
const handleCopyId
const handleViewRaw
const handleOpenGitHub
const githubUrl
const buttonSize
```

### File: `packages/catalog/src/components/catalog/RelatedItems.tsx`

- Size: 5410 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 interfaces, 1 types
- 150 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type RelatedItemType

// Traits/Interfaces
interface RelatedItem
interface RelatedItem
interface RelatedItemsProps
interface RelatedItemsProps

// Constants
const typeIcons : Record<RelatedItemType, React.ReactNode>
const typeColors : Record<RelatedItemType, string>
const displayedItems
const hasMore
```

### File: `packages/catalog/src/components/catalog/SearchBar.tsx`

- Size: 7644 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 2 interfaces, 1 types
- 204 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type EntityType

// Traits/Interfaces
interface SearchBarProps
interface SearchBarProps
interface SearchFilters
interface SearchFilters

// Constants
const entityTypeOptions : Array<{ value: EntityType | "all"; label: string }>
const localFilters
const externalEntityType
const externalDomain
const handleFilterChange
const newFilters
const handleClearAll
const clearedFilters : SearchFilters
const hasActiveFilters
```

### File: `packages/catalog/src/components/catalog/SortDropdown.tsx`

- Size: 6671 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions, 1 interfaces, 1 types
- 181 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type SortOption

// Traits/Interfaces
interface SortDropdownProps
interface SortDropdownProps

// Constants
const defaultOptions : SortOption[]

// Functions
function SortDropdown({
  value = "name",
  direction = "asc",
  onChange,
  options = defaultOptions,
  className,
  size = "md",
  showDirectionToggle = false,
}: SortDropdownProps)
function SortDropdown({
  value = "name",
  direction = "asc",
  onChange,
  options = defaultOptions,
  className,
  size = "md",
  showDirectionToggle = false,
}: SortDropdownProps)

// Constants
const isOpen
const dropdownRef
const combinedValue
const currentOption
const handleClickOutside
const handleSelect
const toggleDirection
const sizeClasses
const optionValue
const isSelected
```

### File: `packages/catalog/src/components/catalog/TreeView.tsx`

- Size: 8357 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions, 2 interfaces
- 291 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface TreeNode
interface TreeNode
interface TreeViewProps
interface TreeViewProps

// Constants
const typeIcons : Record<TreeNode["type"], React.ReactNode>
const typeColors : Record<TreeNode["type"], string>
const expanded
const allIds
const collectIds
const toggleExpand
const next
const expandAllNodes
const allIds
const collectIds
const collapseAllNodes
```

### File: `packages/catalog/src/components/common/EmptyState.tsx`

- Size: 5702 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 203 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface EmptyStateProps
interface EmptyStateProps
```

### File: `packages/catalog/src/components/common/LoadingSkeleton.tsx`

- Size: 5977 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 2 interfaces
- 228 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SkeletonProps

// Functions
function Skeleton({ className }: SkeletonProps)

// Traits/Interfaces
interface CardSkeletonProps
interface CardSkeletonProps
```

### File: `packages/catalog/src/components/common/Pagination.tsx`

- Size: 8482 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 interfaces
- 306 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PaginationProps
interface PaginationProps

// Constants
const generatePageNumbers : (number | "ellipsis")[]
const pages : (number | "ellipsis")[]
const i
const leftSibling
const rightSibling
const i
const pageNumbers
const startItem
const endItem
const isActive
```

### File: `packages/catalog/src/components/common/SearchInput.tsx`

- Size: 10192 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions, 1 interfaces
- 380 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SearchInputProps
interface SearchInputProps

// Constants
const sizeClasses
const iconSizeClasses
const clearButtonSizeClasses
const size
const internalValue
const debounceRef
const value
const setValue
const handleChange
const newValue
const handleClear
const handleKeyDown
const isOpen
const selectedIndex
const containerRef
const filteredSuggestions
const hasSuggestions
const handleClickOutside
const handleChange
const handleSuggestionClick
const handleKeyDown
const suggestion
```

### File: `packages/catalog/src/components/common/Tag.tsx`

- Size: 5047 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 interfaces
- 200 lines (0 code)

**Signatures:**

```typescript
// Constants
const tagVariants

// Traits/Interfaces
interface TagProps
interface TagProps

// Constants
const isClickable
const handleClick
const handleRemove
const handleKeyDown
const visibleTags
const hiddenCount
```

### File: `packages/catalog/src/components/dashboard/BarChart.tsx`

- Size: 3534 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 3 interfaces
- 147 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BarChartData
interface BarChartData
interface BarChartProps
interface BarChartProps

// Constants
const FALLBACK_COLORS

// Traits/Interfaces
interface CustomTooltipProps

// Functions
function CustomTooltip({ active, payload }: CustomTooltipProps)

// Constants
const data
const router
const handleBarClick
const chartData
```

### File: `packages/catalog/src/components/dashboard/MetricCard.tsx`

- Size: 3872 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions, 1 interfaces, 1 types
- 150 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type TrendDirection

// Traits/Interfaces
interface MetricCardProps
interface MetricCardProps

// Functions
function AnimatedValue({ value }: { value: number | string })

// Constants
const displayValue
const numericValue
const isNumeric
const duration
const steps
const stepTime
const increment
const current
const timer

// Functions
function TrendIndicator({ direction, value }: { direction: TrendDirection; value: string })

// Constants
const colors
const icons
```

### File: `packages/catalog/src/components/dashboard/PieChart.tsx`

- Size: 5354 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 4 interfaces
- 217 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PieChartData
interface PieChartData
interface PieChartProps
interface PieChartProps

// Constants
const DEFAULT_COLORS

// Traits/Interfaces
interface CustomTooltipProps

// Functions
function CustomTooltip({ active, payload }: CustomTooltipProps)

// Constants
const data
const total
const percent

// Traits/Interfaces
interface CustomLegendProps
```

### File: `packages/catalog/src/components/dashboard/QuickLinks.tsx`

- Size: 4219 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 interfaces
- 130 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface QuickLinkItem
interface QuickLinkItem
interface QuickLinksProps
interface QuickLinksProps

// Constants
const COLUMN_CLASSES
```

### File: `packages/catalog/src/components/dashboard/RecentActivity.tsx`

- Size: 5650 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 2 interfaces, 1 types
- 162 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type EntityType

// Traits/Interfaces
interface ActivityItem
interface ActivityItem
interface RecentActivityProps
interface RecentActivityProps

// Constants
const TYPE_ICONS : Record<EntityType, React.ReactNode>
const TYPE_COLORS : Record<EntityType, string>

// Functions
function getHref(item: ActivityItem) : string
function formatRelativeTime(dateString: string) : string

// Constants
const date
const now
const diffInSeconds

// Functions
function ActivityRow({ item }: { item: ActivityItem })

// Constants
const displayItems
```

### File: `packages/catalog/src/components/dashboard/StatsOverview.tsx`

- Size: 3262 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 2 interfaces
- 102 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface StatsOverviewProps
interface StatsOverviewProps

// Functions
function formatBytes(bytes: number) : string

// Constants
const k
const sizes
const i

// Functions
function formatDate(dateString: string | null) : string

// Constants
const date

// Traits/Interfaces
interface StatItemProps

// Functions
function StatItem({ icon, label, value }: StatItemProps)
function StatsOverview({
  totalEntities,
  totalFilesIndexed,
  lastIndexTime,
  databaseSize,
  className,
}: StatsOverviewProps)
function StatsOverview({
  totalEntities,
  totalFilesIndexed,
  lastIndexTime,
  databaseSize,
  className,
}: StatsOverviewProps)
```

### File: `packages/catalog/src/components/dashboard/TreemapChart.tsx`

- Size: 5047 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 4 interfaces
- 208 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface TreemapData
interface TreemapData
interface TreemapChartProps
interface TreemapChartProps

// Constants
const DOMAIN_COLORS : Record<string, string>
const DEFAULT_COLORS

// Traits/Interfaces
interface CustomTooltipProps

// Functions
function CustomTooltip({ active, payload }: CustomTooltipProps)

// Constants
const item
const data

// Traits/Interfaces
interface TreemapContentProps

// Constants
const domainName
const baseColor
const safeBaseColor
const color
const showText
const num
const amt
const R
const G
const B
const itemColor
```

### File: `packages/catalog/src/components/decorations/BotanicalDecor.tsx`

- Size: 28593 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 1 interfaces
- 680 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BotanicalDecorProps

// Functions
function BotanicalDecor({ variant, className = "", size = 80 }: BotanicalDecorProps)
function BotanicalDecor({ variant, className = "", size = 80 }: BotanicalDecorProps)

// Constants
const renderDecor
const rad
const x
const y
```

### File: `packages/catalog/src/components/decorations/BrassPipeBorder.tsx`

- Size: 42019 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 763 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BrassPipeBorderProps

// Functions
function BrassPipeBorder({ side, className = "" }: BrassPipeBorderProps)
function BrassPipeBorder({ side, className = "" }: BrassPipeBorderProps)

// Constants
const isLeft
const rad
const x1
const y1
const x2
const y2
```

### File: `packages/catalog/src/components/decorations/CardCornerFlourish.tsx`

- Size: 3807 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 131 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface CardCornerFlourishProps

// Functions
function CardCornerFlourish({ position, size = 28, className = "" }: CardCornerFlourishProps)
function CardCornerFlourish({ position, size = 28, className = "" }: CardCornerFlourishProps)

// Constants
const rotations
const positionStyles
```

### File: `packages/catalog/src/components/decorations/GearCluster.tsx`

- Size: 7082 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 interfaces
- 194 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface GearClusterProps

// Functions
function generateGearPath(cx: number, cy: number, innerRadius: number, outerRadius: number, teeth: number) : string

// Constants
const points : string[]
const angleStep
const i
const angle
const radius
const x
const y

// Functions
function GearCluster({ position, className = "" }: GearClusterProps)
function GearCluster({ position, className = "" }: GearClusterProps)

// Constants
const isLeft
const colors
const c
const teeth
```

### File: `packages/catalog/src/components/decorations/MechanicalBee.tsx`

- Size: 13137 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 227 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface MechanicalBeeProps

// Functions
function MechanicalBee({ size = 64, className = "", style }: MechanicalBeeProps)
function MechanicalBee({ size = 64, className = "", style }: MechanicalBeeProps)

// Constants
const beePositions
```

### File: `packages/catalog/src/components/layout/Breadcrumb.tsx`

- Size: 3362 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 interfaces
- 127 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BreadcrumbItem
interface BreadcrumbItem
interface BreadcrumbProps
interface BreadcrumbProps

// Constants
const allItems : BreadcrumbItem[]
const isFirst
const isLast
```

### File: `packages/catalog/src/components/layout/Footer.tsx`

- Size: 4223 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 interfaces
- 111 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface FooterLink

// Constants
const defaultLinks : FooterLink[]

// Traits/Interfaces
interface FooterProps
interface FooterProps

// Constants
const linkClasses
```

### File: `packages/catalog/src/components/layout/Header.tsx`

- Size: 10205 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 2 interfaces
- 259 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface NavItem

// Constants
const navItems : NavItem[]

// Traits/Interfaces
interface HeaderProps
interface HeaderProps

// Constants
const router
const isMenuOpen
const searchQuery
const allNavItems
const handleSearchSubmit
const searchUrl
const handleKeyDown
const searchInput
```

### File: `packages/catalog/src/components/layout/PageContainer.tsx`

- Size: 2455 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 92 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PageContainerProps
interface PageContainerProps

// Constants
const maxWidthClasses
const paddingClasses
const sidebarCollapsed
```

### File: `packages/catalog/src/components/layout/Sidebar.tsx`

- Size: 9068 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 3 interfaces
- 246 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SidebarSection
interface SidebarItem

// Constants
const defaultSections : SidebarSection[]

// Traits/Interfaces
interface SidebarProps
interface SidebarProps

// Constants
const expandedSections
const toggleSection
const next
```

### File: `packages/catalog/src/components/markdown/CodeBlock.tsx`

- Size: 6072 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 1 interfaces
- 196 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface CodeBlockProps

// Constants
const copied
const language
const match
const languageDisplayName
const languageMap : Record<string, string>
const codeText
const props
const handleCopy
const lines
const lineCount
```

### File: `packages/catalog/src/components/markdown/FrontmatterDisplay.tsx`

- Size: 8749 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions, 3 interfaces, 1 types
- 265 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type FrontmatterValue

// Traits/Interfaces
interface FrontmatterDisplayProps
interface TypeBadgeProps

// Functions
function TypeBadge({ type }: TypeBadgeProps)

// Constants
const typeConfig : Record<string, { icon: React.ReactNode; color: string }>
const config

// Functions
function detectType(value: FrontmatterValue) : string

// Constants
const dateRegex

// Traits/Interfaces
interface ValueRendererProps

// Constants
const isExpanded
const type
const allPrimitive
const isCollapsed
const entries
```

### File: `packages/catalog/src/components/markdown/ImageHandler.tsx`

- Size: 6121 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions, 2 interfaces
- 234 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ImageHandlerProps
interface LightboxProps

// Functions
function resolveImageSrc(src: string | undefined, basePath: string) : string

// Constants
const baseSegments
const upCount
const remaining
const newBase
const handleKeyDown
const isLoaded
const hasError
const showLightbox
const resolvedSrc
const handleLoad
const handleError
const handleOpenLightbox
const handleCloseLightbox
```

### File: `packages/catalog/src/components/markdown/LinkHandler.tsx`

- Size: 6315 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions, 2 interfaces
- 250 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface LinkHandlerProps
interface LinkConfig

// Functions
function analyzeLinkHref(href: string | undefined, basePath: string) : LinkConfig

// Constants
const resolvedPath
const baseSegments
const upCount
const remaining
const newBase

// Functions
function ExternalLink({
  href,
  children,
  className = '',
}: {
  href: string;
  children: React.ReactNode;
  className?: string;
})
function InternalLink({
  href,
  children,
  className = '',
}: {
  href: string;
  children: React.ReactNode;
  className?: string;
})
function AnchorLink({
  href,
  children,
  className = '',
}: {
  href: string;
  children: React.ReactNode;
  className?: string;
})

// Constants
const handleClick
const id
const element

// Functions
function LinkHandler({
  href,
  children,
  className = '',
  basePath = '',
}: LinkHandlerProps)
function LinkHandler({
  href,
  children,
  className = '',
  basePath = '',
}: LinkHandlerProps)

// Constants
const linkConfig
```

### File: `packages/catalog/src/components/markdown/MarkdownRenderer.tsx`

- Size: 8608 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions, 1 interfaces
- 290 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface MarkdownRendererProps

// Constants
const HeadingComponent
const text
const id
const commonProps
const anchorLink
const isInline
const filename
```

### File: `packages/catalog/src/components/markdown/TableOfContents.tsx`

- Size: 6503 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 15 functions, 4 interfaces
- 264 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface Heading
interface TableOfContentsProps
interface TocItemProps

// Functions
function slugify(text: string) : string
function extractHeadings(markdown: string, maxDepth: number = 3) : Heading[]

// Constants
const headings : Heading[]
const lines
const slugCounts : Record<string, number>
const match
const level
const text
const slug
const existingCount

// Traits/Interfaces
interface HeadingNode

// Functions
function buildTree(headings: Heading[]) : HeadingNode[]

// Constants
const root : HeadingNode[]
const stack : HeadingNode[]
const node : HeadingNode
const lastItem
const parent

// Functions
function TocItem({ heading, isActive, onClick, children }: TocItemProps)

// Constants
const indent
const activeId
const headings
const tree
const observer
const element
const handleClick
const element
```

### File: `packages/catalog/src/components/ui/badge.tsx`

- Size: 2424 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 61 lines (0 code)

**Signatures:**

```typescript
// Constants
const badgeVariants

// Traits/Interfaces
interface BadgeProps
interface BadgeProps

// Functions
function Badge({ className, variant, style, ...props }: BadgeProps)

// Constants
const steampunkStyle
```

### File: `packages/catalog/src/components/ui/button.tsx`

- Size: 2667 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 73 lines (0 code)

**Signatures:**

```typescript
// Constants
const buttonVariants

// Traits/Interfaces
interface ButtonProps
interface ButtonProps

// Constants
const Button
const Comp
const brassStyle
```

### File: `packages/catalog/src/components/ui/card.tsx`

- Size: 2441 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 98 lines (0 code)

**Signatures:**

```typescript
// Constants
const Card
const CardHeader
const CardTitle
```

### File: `packages/catalog/src/components/ui/input.tsx`

- Size: 774 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 22 lines (0 code)

**Signatures:**

```typescript
// Constants
const Input
```

### File: `packages/catalog/src/components/ui/separator.tsx`

- Size: 779 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const Separator
```

### File: `packages/catalog/src/components/ui/skeleton.tsx`

- Size: 310 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 14 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SkeletonProps

// Functions
function Skeleton({ className, ...props }: SkeletonProps)
```

### File: `packages/catalog/src/lib/api/types.ts`

- Size: 7923 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 31 interfaces
- 383 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PaginationMeta
interface PaginationMeta
interface ApiResponse
interface ApiResponse
interface ApiError
interface ApiError
interface ListQueryParams
interface ListQueryParams
interface SearchQueryParams
interface SearchQueryParams
interface SearchResultItem
interface SearchResultItem
interface SearchResponse
interface SearchResponse
interface ProcessQueryParams
interface ProcessQueryParams
interface ProcessListItem
interface ProcessListItem
interface ProcessTask
interface ProcessTask
interface ProcessIO
interface ProcessIO
interface ProcessDetail
interface ProcessDetail
interface DomainListItem
interface DomainListItem
interface SpecializationSummary
interface SpecializationSummary
interface DomainDetail
interface DomainDetail
interface SpecializationQueryParams
interface SpecializationQueryParams
interface SpecializationListItem
interface SpecializationListItem
interface AgentSummary
interface AgentSummary
interface SkillSummary
interface SkillSummary
interface SpecializationDetail
interface SpecializationDetail
interface SkillQueryParams
interface SkillQueryParams
interface SkillListItem
interface SkillListItem
interface SkillDetail
interface SkillDetail
interface AgentQueryParams
interface AgentQueryParams
interface AgentListItem
interface AgentListItem
interface AgentDetail
interface AgentDetail
interface EntityDistribution
interface EntityDistribution
interface RecentActivityItem
interface RecentActivityItem
interface AnalyticsResponse
interface AnalyticsResponse
interface ReindexRequest
interface ReindexRequest
interface ReindexResponse
interface ReindexResponse
```

### File: `packages/catalog/src/lib/api/utils.ts`

- Size: 8019 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 21 functions
- 293 lines (0 code)

**Signatures:**

```typescript
// Constants
const DEFAULT_LIMIT
const MAX_LIMIT
const DEFAULT_OFFSET

// Functions
function parseListQueryParams(searchParams: URLSearchParams) : ListQueryParams
function parseListQueryParams(searchParams: URLSearchParams) : ListQueryParams

// Constants
const limit
const offset
const sort
const order

// Functions
function parseIntParam(value: string | null, defaultValue: number) : number
function parseIntParam(value: string | null, defaultValue: number) : number

// Constants
const parsed

// Functions
function parseOrderParam(value: string | null) : 'asc' | 'desc' | undefined
function parseOrderParam(value: string | null) : 'asc' | 'desc' | undefined
function parseArrayParam(value: string | null) : string[]
function parseArrayParam(value: string | null) : string[]
function createSuccessResponse(
  data: T,
  meta?: PaginationMeta,
  status: number = 200
) : NextResponse<ApiResponse<T>>
function createSuccessResponse(
  data: T,
  meta?: PaginationMeta,
  status: number = 200
) : NextResponse<ApiResponse<T>>

// Constants
const response : ApiResponse<T>

// Functions
function createPaginatedResponse(
  data: T[],
  total: number,
  limit: number,
  offset: number
) : NextResponse<ApiResponse<T[]>>
function createPaginatedResponse(
  data: T[],
  total: number,
  limit: number,
  offset: number
) : NextResponse<ApiResponse<T[]>>

// Constants
const meta : PaginationMeta

// Functions
function createErrorResponse(
  code: string,
  message: string,
  status: number = 400,
  details?: Record<string, unknown>
) : NextResponse<ApiResponse<never>>
function createErrorResponse(
  code: string,
  message: string,
  status: number = 400,
  details?: Record<string, unknown>
) : NextResponse<ApiResponse<never>>

// Constants
const error : ApiError

// Functions
function notFoundResponse(resource: string, identifier: string | number) : NextResponse<ApiResponse<never>>
function notFoundResponse(resource: string, identifier: string | number) : NextResponse<ApiResponse<never>>
function badRequestResponse(message: string, details?: Record<string, unknown>) : NextResponse<ApiResponse<never>>
function badRequestResponse(message: string, details?: Record<string, unknown>) : NextResponse<ApiResponse<never>>
function internalErrorResponse(error: unknown) : NextResponse<ApiResponse<never>>
function internalErrorResponse(error: unknown) : NextResponse<ApiResponse<never>>

// Constants
const message

// Functions
function validationErrorResponse(
  field: string,
  message: string
) : NextResponse<ApiResponse<never>>
function validationErrorResponse(
  field: string,
  message: string
) : NextResponse<ApiResponse<never>>
function requireQueryParam(
  searchParams: URLSearchParams,
  name: string
) : { value: string } | { error: NextResponse<ApiResponse<never>> }
function requireQueryParam(
  searchParams: URLSearchParams,
  name: string
) : { value: string } | { error: NextResponse<ApiResponse<never>> }

// Constants
const value

// Functions
function validateSlug(slug: string | undefined) : { valid: true; slug: string } | { valid: false; error: NextResponse<ApiResponse<never>> }
function validateSlug(slug: string | undefined) : { valid: true; slug: string } | { valid: false; error: NextResponse<ApiResponse<never>> }
function validateId(id: string | undefined) : { valid: true; id: number } | { valid: false; error: NextResponse<ApiResponse<never>> }
function validateId(id: string | undefined) : { valid: true; id: number } | { valid: false; error: NextResponse<ApiResponse<never>> }

// Constants
const numId

// Functions
function safeJsonParse(value: string | null | undefined, defaultValue: T) : T
function safeJsonParse(value: string | null | undefined, defaultValue: T) : T
function toCamelCase(obj: T) : Record<string, unknown>
function toCamelCase(obj: T) : Record<string, unknown>

// Constants
const result : Record<string, unknown>
const camelKey

// Functions
function getRouteParams(
  _request: NextRequest,
  params: Record<string, string | string[]>
) : Record<string, string>
function getRouteParams(
  _request: NextRequest,
  params: Record<string, string | string[]>
) : Record<string, string>

// Constants
const result : Record<string, string>

// Functions
function mapSortField(
  field: string | undefined,
  allowedFields: Record<string, string>,
  defaultField: string
) : string
function mapSortField(
  field: string | undefined,
  allowedFields: Record<string, string>,
  defaultField: string
) : string
function buildOrderClause(
  sort: string | undefined,
  order: 'asc' | 'desc' | undefined,
  allowedFields: Record<string, string>,
  defaultField: string
) : string
function buildOrderClause(
  sort: string | undefined,
  order: 'asc' | 'desc' | undefined,
  allowedFields: Record<string, string>,
  defaultField: string
) : string

// Constants
const sortField
const sortOrder
```

### File: `packages/catalog/src/lib/db/client.ts`

- Size: 7641 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 1 classes
- 303 lines (0 code)

**Signatures:**

```typescript
// Constants
const isInCatalogDir
const DEFAULT_DB_PATH

// Structs/Classes
class CatalogDatabase
class CatalogDatabase

// Constants
const dbDir
const currentVersion
const domainsCount
const specializationsCount
const agentsCount
const skillsCount
const processesCount
const metadata
const databaseSize
const stats

// Functions
function getDatabase(options?: Partial<DatabaseClientOptions>) : CatalogDatabase
function getDatabase(options?: Partial<DatabaseClientOptions>) : CatalogDatabase
function initializeDatabase(options?: Partial<DatabaseClientOptions>) : CatalogDatabase
function initializeDatabase(options?: Partial<DatabaseClientOptions>) : CatalogDatabase

// Constants
const db

// Functions
function closeDatabase() : void
function closeDatabase() : void

// Constants
const instance

// Functions
function resetCatalogDatabase(options?: Partial<DatabaseClientOptions>) : void
function resetCatalogDatabase(options?: Partial<DatabaseClientOptions>) : void

// Constants
const db

// Functions
function getDatabasePath() : string
function getDatabasePath() : string
function databaseExists(dbPath?: string) : boolean
function databaseExists(dbPath?: string) : boolean
```

### File: `packages/catalog/src/lib/db/indexer.ts`

- Size: 24934 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 20 functions, 1 classes, 3 interfaces
- 856 lines (0 code)

**Signatures:**

```typescript
// Constants
const DEFAULT_LIBRARY_PATHS
const DEFAULT_BATCH_SIZE

// Functions
function createFileSystemAdapter() : FileSystem

// Constants
const stats

// Traits/Interfaces
interface ParsedAgentData
interface ParsedSkillData
interface ParsedProcessData

// Structs/Classes
class CatalogIndexer
class CatalogIndexer

// Constants
const startTime
const stats
const scanResults
const parsedAgents
const parsedSkills
const parsedProcesses
const errorMessage
const domains : DomainInfo[]
const specializations : SpecializationInfo[]
const agentFiles : string[]
const skillFiles : string[]
const processFiles : string[]
const fullPath
const basePath
const result
const stmt
const count
const pathParts
const domainsIdx
const category
const stmt
const getDomainId
const count
const indexSpec
const domainId : number | null
const domainRow
const results : ParsedAgentData[]
const getFileMtime
const i
const filePath
const mtime
const tracking
const result
const stmt
const getSpecId
const getDomainId
const updateTracking
const indexed
const i
const batch
const specId : number | null
const domainId : number | null
const specRow
const domainRow
const content
const results : ParsedSkillData[]
const getFileMtime
const i
const filePath
const mtime
const tracking
const result
const stmt
const getSpecId
const getDomainId
const updateTracking
const indexed
const i
const batch
const specId : number | null
const domainId : number | null
const specRow
const domainRow
const content
const results : ParsedProcessData[]
const getFileMtime
const i
const filePath
const mtime
const tracking
const result
const stmt
const updateTracking
const indexed
const i
const batch
const stats

// Functions
function runFullIndex(
  baseDir: string,
  options?: IndexerOptions
) : Promise<IndexResult>
function runFullIndex(
  baseDir: string,
  options?: IndexerOptions
) : Promise<IndexResult>

// Constants
const indexer

// Functions
function runIncrementalIndex(
  baseDir: string,
  options?: IndexerOptions
) : Promise<IndexResult>
function runIncrementalIndex(
  baseDir: string,
  options?: IndexerOptions
) : Promise<IndexResult>

// Constants
const indexer

// Functions
function needsRebuild() : boolean
function needsRebuild() : boolean

// Constants
const db
const stats
```

### File: `packages/catalog/src/lib/db/init.ts`

- Size: 7311 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions, 2 interfaces
- 259 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface InitOptions
interface InitOptions
interface InitResult
interface InitResult

// Constants
const isInitialized
const initPromise : Promise<InitResult> | null

// Functions
function ensureDatabaseInitialized(options: InitOptions = {}) : Promise<InitResult>
function ensureDatabaseInitialized(options: InitOptions = {}) : Promise<InitResult>

// Constants
const db
const stats
const result

// Functions
function performInitialization(options: InitOptions) : Promise<InitResult>

// Constants
const startTime
const errors : string[]
const log
const db
const stats
const isEmpty
const shouldReindex
const baseDir
const indexer
const total
const percent
const indexResult
const finalStats
const message

// Functions
function needsInitialization() : boolean
function needsInitialization() : boolean

// Constants
const db
const stats

// Functions
function resetInitializationState() : void
function resetInitializationState() : void
function getInitializationStatus() : {
  isInitialized: boolean;
  isInitializing: boolean;
}
function getInitializationStatus() : {
  isInitialized: boolean;
  isInitializing: boolean;
}
function withDatabaseInit(
  handler: () => Promise<T>,
  options: InitOptions = {}
) : Promise<T>
function withDatabaseInit(
  handler: () => Promise<T>,
  options: InitOptions = {}
) : Promise<T>
function createInitializedHandler(
  handler: (...args: TArgs) => Promise<TResult>,
  options: InitOptions = {}
) : (...args: TArgs) => Promise<TResult>
function createInitializedHandler(
  handler: (...args: TArgs) => Promise<TResult>,
  options: InitOptions = {}
) : (...args: TArgs) => Promise<TResult>
```

### File: `packages/catalog/src/lib/db/queries.ts`

- Size: 23085 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 14 functions, 2 classes
- 879 lines (0 code)

**Signatures:**

```typescript
// Structs/Classes
class QueryBuilder
class QueryBuilder

// Constants
const parts : string[]
const options
const sql
const originalSelect
const originalLimit
const originalOffset
const result
const totalItems
const totalPages
const data
const inValues
const placeholders
const notInValues
const notInPlaceholders

// Structs/Classes
class CatalogQueries
class CatalogQueries

// Constants
const builder
const builder
const builder
const limit
const types
const typeFilter
const sql
const rows
const sql
const rows
const sql
const rows
const sql
const rows
const sql
const rows
const entries : CatalogEntryView[]
const filtered
const value
const searchLower
const aVal
const bVal
const cmp
const options
const totalItems
const totalPages
const start
const data

// Functions
function applyFilter(value: unknown, operator: FilterOperator, filterValue: unknown) : boolean
function createCatalogQueries(db?: CatalogDatabase) : CatalogQueries
function createCatalogQueries(db?: CatalogDatabase) : CatalogQueries
function createQueryBuilder(tableName: string, db?: CatalogDatabase) : QueryBuilder<T>
function createQueryBuilder(tableName: string, db?: CatalogDatabase) : QueryBuilder<T>

// Constants
const database
```

### File: `packages/catalog/src/lib/db/schema.ts`

- Size: 17732 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 535 lines (0 code)

**Signatures:**

```typescript
// Constants
const SCHEMA_VERSION
const CREATE_TABLES_SQL
const CREATE_INDEXES_SQL
const CREATE_FTS_SQL
const CREATE_TRIGGERS_SQL

// Functions
function initializeSchema(db: Database.Database) : void
function initializeSchema(db: Database.Database) : void

// Constants
const versionStmt

// Functions
function getSchemaVersion(db: Database.Database) : number
function getSchemaVersion(db: Database.Database) : number

// Constants
const row

// Functions
function needsMigration(db: Database.Database) : boolean
function needsMigration(db: Database.Database) : boolean

// Constants
const currentVersion

// Functions
function runMigrations(db: Database.Database) : void
function runMigrations(db: Database.Database) : void

// Constants
const currentVersion

// Functions
function resetDatabase(db: Database.Database) : void
function resetDatabase(db: Database.Database) : void

// Constants
const dropStatements
const dropTriggers

// Functions
function rebuildFTSIndexes(db: Database.Database) : void
function rebuildFTSIndexes(db: Database.Database) : void
function rebuildCatalogSearch(db: Database.Database) : void
function rebuildCatalogSearch(db: Database.Database) : void
function getTableNames() : string[]
function getTableNames() : string[]
function getFTSTableNames() : string[]
function getFTSTableNames() : string[]
function getIndexNames() : string[]
function getIndexNames() : string[]
function getSchemaInfo() : {
  tables: Array<{ name: string; description: string }>;
  indexes: Array<{ name: string; table: string }>;
  fts: Array<{ name: string; description: string }>;
}
function getSchemaInfo() : {
  tables: Array<{ name: string; description: string }>;
  indexes: Array<{ name: string; table: string }>;
  fts: Array<{ name: string; description: string }>;
}
```

### File: `packages/catalog/src/lib/db/types.ts`

- Size: 6818 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 19 interfaces, 4 types
- 323 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface DomainRow
interface DomainRow
interface SpecializationRow
interface SpecializationRow
interface AgentRow
interface AgentRow
interface SkillRow
interface SkillRow
interface ProcessRow
interface ProcessRow
interface FileTrackingRow
interface FileTrackingRow
interface FTSSearchRow
interface FTSSearchRow
interface SearchResult
interface SearchResult

// Type Aliases
type FilterOperator

// Traits/Interfaces
interface FilterCondition
interface FilterCondition

// Type Aliases
type SortDirection

// Traits/Interfaces
interface SortSpec
interface SortSpec
interface PaginationOptions
interface PaginationOptions
interface QueryOptions
interface QueryOptions
interface PaginatedResult
interface PaginatedResult

// Type Aliases
type IndexProgressCallback

// Traits/Interfaces
interface IndexProgress
interface IndexProgress
interface IndexResult
interface IndexResult
interface IndexerOptions
interface IndexerOptions
interface DatabaseClientOptions
interface DatabaseClientOptions
interface DatabaseStats
interface DatabaseStats

// Type Aliases
type CatalogEntryType

// Traits/Interfaces
interface CatalogEntryView
interface CatalogEntryView
```

### File: `packages/catalog/src/lib/hooks/useApi.ts`

- Size: 15309 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 40 functions, 7 interfaces, 1 types
- 532 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface UseQueryOptions
interface UseQueryOptions
interface UseQueryResult
interface UseQueryResult
interface UsePaginatedQueryResult
interface UsePaginatedQueryResult
interface CatalogQueryParams
interface CatalogQueryParams

// Functions
function useQuery(
  endpoint: string,
  options: UseQueryOptions<T> = {}
) : UseQueryResult<T>
function useQuery(
  endpoint: string,
  options: UseQueryOptions<T> = {}
) : UseQueryResult<T>

// Constants
const options
const data
const error
const isLoading
const onSuccessRef
const onErrorRef
const fetchData
const response
const json : ApiResponse<T>
const errorObj
const interval

// Functions
function usePaginatedQuery(
  baseEndpoint: string,
  params: CatalogQueryParams = {},
  options: UseQueryOptions<T[]> = {}
) : UsePaginatedQueryResult<T>
function usePaginatedQuery(
  baseEndpoint: string,
  params: CatalogQueryParams = {},
  options: UseQueryOptions<T[]> = {}
) : UsePaginatedQueryResult<T>

// Constants
const pagination
const allData
const memoizedParams
const buildEndpoint
const searchParams
const queryString
const queryResult
const fetchPagination
const response
const json
const loadMore

// Functions
function useAgents(params: CatalogQueryParams = {})
function useAgents(params: CatalogQueryParams = {})
function useAgent(slug: string, options: UseQueryOptions<AgentDetail> = {})
function useAgent(slug: string, options: UseQueryOptions<AgentDetail> = {})
function useSkills(params: CatalogQueryParams = {})
function useSkills(params: CatalogQueryParams = {})
function useSkill(slug: string, options: UseQueryOptions<SkillDetail> = {})
function useSkill(slug: string, options: UseQueryOptions<SkillDetail> = {})
function useProcesses(params: CatalogQueryParams = {})
function useProcesses(params: CatalogQueryParams = {})
function useProcess(id: string | number, options: UseQueryOptions<ProcessDetail> = {})
function useProcess(id: string | number, options: UseQueryOptions<ProcessDetail> = {})
function useDomains(params: CatalogQueryParams = {})
function useDomains(params: CatalogQueryParams = {})
function useDomain(slug: string, options: UseQueryOptions<DomainDetail> = {})
function useDomain(slug: string, options: UseQueryOptions<DomainDetail> = {})
function useSpecializations(params: CatalogQueryParams = {})
function useSpecializations(params: CatalogQueryParams = {})
function useSpecialization(slug: string, options: UseQueryOptions<SpecializationDetail> = {})
function useSpecialization(slug: string, options: UseQueryOptions<SpecializationDetail> = {})

// Traits/Interfaces
interface SearchParams
interface SearchParams

// Functions
function useSearch(params: SearchParams, options: UseQueryOptions<SearchResultItem[]> = {})
function useSearch(params: SearchParams, options: UseQueryOptions<SearchResultItem[]> = {})

// Constants
const memoizedParams
const buildEndpoint
const searchParams

// Functions
function useDebouncedSearch(
  params: SearchParams,
  delay: number = 300,
  options: UseQueryOptions<SearchResultItem[]> = {}
)
function useDebouncedSearch(
  params: SearchParams,
  delay: number = 300,
  options: UseQueryOptions<SearchResultItem[]> = {}
)

// Constants
const debouncedQuery
const timer

// Functions
function useAnalytics(options: UseQueryOptions<AnalyticsResponse> = {})
function useAnalytics(options: UseQueryOptions<AnalyticsResponse> = {})

// Type Aliases
type EntityType

// Functions
function useEntityDetail(
  type: EntityType,
  identifier: string | number,
  options: UseQueryOptions<T> = {}
)
function useEntityDetail(
  type: EntityType,
  identifier: string | number,
  options: UseQueryOptions<T> = {}
)

// Constants
const endpoint

// Traits/Interfaces
interface UseMutationOptions
interface UseMutationOptions
interface UseMutationResult
interface UseMutationResult

// Functions
function useMutation(
  mutationFn: (variables: TVariables) => Promise<TData>,
  options: UseMutationOptions<TData, TVariables> = {}
) : UseMutationResult<TData, TVariables>
function useMutation(
  mutationFn: (variables: TVariables) => Promise<TData>,
  options: UseMutationOptions<TData, TVariables> = {}
) : UseMutationResult<TData, TVariables>

// Constants
const data
const error
const isLoading
const optionsRef
const mutationFnRef
const reset
const mutate
const result
const errorObj

// Functions
function useReindex(options: UseMutationOptions<void, { force?: boolean }> = {})
function useReindex(options: UseMutationOptions<void, { force?: boolean }> = {})

// Constants
const response
const json
```

### File: `packages/catalog/src/lib/parsers/agent.ts`

- Size: 9862 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions, 1 interfaces
- 390 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface AgentParserOptions
interface AgentParserOptions

// Constants
const DEFAULT_OPTIONS : AgentParserOptions

// Functions
function parseAgentContent(
  content: string,
  filePath: string = '',
  options: AgentParserOptions = {}
) : ParseResult<ParsedAgent>
function parseAgentContent(
  content: string,
  filePath: string = '',
  options: AgentParserOptions = {}
) : ParseResult<ParsedAgent>

// Constants
const opts
const frontmatterResult
const frontmatterResult
const sections : MarkdownSection[]
const sectionsResult
const agent : ParsedAgent

// Functions
function parseAgentFile(
  filePath: string,
  readFile: (path: string) => Promise<string>,
  options: AgentParserOptions = {}
) : Promise<ParseResult<ParsedAgent>>
function parseAgentFile(
  filePath: string,
  readFile: (path: string) => Promise<string>,
  options: AgentParserOptions = {}
) : Promise<ParseResult<ParsedAgent>>

// Constants
const content
const errorMessage

// Functions
function normalizeAgentMetadata(
  metadata: AgentMetadata | undefined,
  filePath: string
) : AgentMetadata

// Constants
const normalized : AgentMetadata

// Functions
function extractResponsibilities(sections: MarkdownSection[]) : string[]

// Constants
const responsibilities : string[]
const responsibilitiesSection
const items
const roleSection

// Functions
function extractRequiredSkills(sections: MarkdownSection[]) : string[]

// Constants
const skills : string[]
const skillSectionNames
const section

// Functions
function extractCollaboration(sections: MarkdownSection[]) : string[]

// Constants
const collaboration : string[]
const collaborationSectionNames
const section

// Functions
function getDirectory(filePath: string) : string

// Constants
const normalized
const lastSlash

// Functions
function validateAgent(agent: ParsedAgent) : {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
function validateAgent(agent: ParsedAgent) : {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// Constants
const errors : string[]
const warnings : string[]

// Functions
function agentToCatalogEntry(agent: ParsedAgent) : {
  id: string;
  type: 'agent';
  name: string;
  description: string;
  path: string;
  domain?: string;
  specialization?: string;
  category?: string;
  phase?: number;
  tags: string[];
  metadata: Record<string, unknown>;
}
function agentToCatalogEntry(agent: ParsedAgent) : {
  id: string;
  type: 'agent';
  name: string;
  description: string;
  path: string;
  domain?: string;
  specialization?: string;
  category?: string;
  phase?: number;
  tags: string[];
  metadata: Record<string, unknown>;
}
function generateAgentSummary(agent: ParsedAgent) : string
function generateAgentSummary(agent: ParsedAgent) : string

// Constants
const parts : string[]
```

### File: `packages/catalog/src/lib/parsers/ast.ts`

- Size: 14184 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 21 functions
- 531 lines (0 code)

**Signatures:**

```typescript
// Constants
const PATTERNS

// Functions
function parseAST(content: string, filePath: string = '') : ParseResult<ParsedAST>
function parseAST(content: string, filePath: string = '') : ParseResult<ParsedAST>

// Constants
const result : ParsedAST
const errorMessage

// Functions
function parseImports(content: string) : ParsedAST['imports']

// Constants
const imports : ParsedAST['imports']
const match
const namedImports
const defaultImport
const additionalNamed
const source
const specifiers : string[]
const names
const names

// Functions
function parseExports(content: string, filePath: string) : ExportInfo[]

// Constants
const exports : ExportInfo[]
const match
const name
const lineNumber
const asyncKeyword
const name
const lineNumber
const exportedValue
const lineNumber
const name

// Functions
function parseDefineTasks(content: string, filePath: string) : DefineTaskCall[]

// Constants
const tasks : DefineTaskCall[]
const match
const variableName
const taskName
const lineNumber
const taskContent
const task : DefineTaskCall
const kindMatch
const titleMatch
const templateTitleMatch
const descMatch
const labelsMatch
const agentNameMatch
const roleMatch
const taskTextMatch
const inputPathMatch
const outputPathMatch

// Functions
function extractDefineTaskContent(content: string, startIndex: number) : string

// Constants
const depth
const started
const endIndex
const i
const char

// Functions
function parseFunctions(
  content: string,
  filePath: string
) : ParsedAST['functions']

// Constants
const functions : ParsedAST['functions']
const seen
const match
const fullMatch
const asyncKeyword
const name
const params
const lineNumber
const fullMatch
const name
const asyncKeyword
const lineNumber

// Functions
function parseParams(paramsStr: string) : string[]

// Constants
const params : string[]
const current
const depth
const param
const lastParam

// Functions
function extractParamName(paramDecl: string) : string

// Constants
const parts
const withoutDefault
const typeParts
const withoutType

// Functions
function getLineNumber(content: string, index: number) : number

// Constants
const beforeMatch

// Functions
function usesDefineTask(content: string) : boolean
function usesDefineTask(content: string) : boolean
function getTaskNames(content: string) : string[]
function getTaskNames(content: string) : string[]

// Constants
const result

// Functions
function hasProcessFunction(content: string) : boolean
function hasProcessFunction(content: string) : boolean

// Constants
const result

// Functions
function extractProcessId(content: string, filePath: string) : string
function extractProcessId(content: string, filePath: string) : string

// Constants
const processMatch
const normalizedPath
const parts
const fileName
const parentDir
```

### File: `packages/catalog/src/lib/parsers/directory.ts`

- Size: 14140 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions, 2 interfaces
- 564 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface FileSystem
interface FileSystem
interface DirectoryParserOptions
interface DirectoryParserOptions

// Constants
const DEFAULT_PATTERNS

// Functions
function parseDirectoryStructure(
  baseDir: string,
  fs: FileSystem,
  options: DirectoryParserOptions = {}
) : Promise<ParseResult<DirectoryScanResult>>
function parseDirectoryStructure(
  baseDir: string,
  fs: FileSystem,
  options: DirectoryParserOptions = {}
) : Promise<ParseResult<DirectoryScanResult>>

// Constants
const patterns
const maxDepth
const result : DirectoryScanResult
const domainsPath
const specializationsPath
const methodologiesPath
const errorMessage

// Functions
function scanDomains(
  domainsPath: string,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS,
  maxDepth: number
) : Promise<DomainInfo[]>

// Constants
const domains : DomainInfo[]
const topLevelDirs
const topDirPath
const stat
const domain

// Functions
function scanDomain(
  domainPath: string,
  name: string,
  category: string | undefined,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS,
  depth: number
) : Promise<DomainInfo | null>

// Constants
const domain : DomainInfo
const entries
const entryPath
const stat
const spec

// Functions
function scanTopLevelSpecializations(
  specializationsPath: string,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS,
  maxDepth: number
) : Promise<SpecializationInfo[]>

// Constants
const specializations : SpecializationInfo[]
const entries
const entryPath
const stat
const spec

// Functions
function scanSpecialization(
  specPath: string,
  name: string,
  domain: string | undefined,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS,
  depth: number
) : Promise<SpecializationInfo | null>

// Constants
const spec : SpecializationInfo
const entries

// Functions
function scanAgentsDirectory(
  agentsPath: string,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS
) : Promise<string[]>

// Constants
const agents : string[]
const entries
const entryPath
const stat
const agentFile

// Functions
function scanSkillsDirectory(
  skillsPath: string,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS
) : Promise<string[]>

// Constants
const skills : string[]
const entries
const entryPath
const stat
const skillFile

// Functions
function scanMethodologies(
  methodologiesPath: string,
  fs: FileSystem,
  patterns: typeof DEFAULT_PATTERNS
) : Promise<string[]>

// Constants
const processes : string[]

// Functions
function scanDir(dirPath: string, depth: number) : Promise<void>

// Constants
const entries
const entryPath
const stat

// Functions
function countAgents(result: DirectoryScanResult) : number

// Constants
const count

// Functions
function countSkills(result: DirectoryScanResult) : number

// Constants
const count

// Functions
function normalizePath(path: string) : string
function getRelativePath(filePath: string, baseDir: string) : string
function getRelativePath(filePath: string, baseDir: string) : string

// Constants
const normalizedFile
const normalizedBase

// Functions
function extractDomainFromPath(filePath: string) : string | undefined
function extractDomainFromPath(filePath: string) : string | undefined

// Constants
const normalized
const domainsMatch

// Functions
function extractSpecializationFromPath(filePath: string) : string | undefined
function extractSpecializationFromPath(filePath: string) : string | undefined

// Constants
const normalized
const domainsMatch
const potentialSpec
const specMatch

// Functions
function extractNameFromPath(filePath: string) : string
function extractNameFromPath(filePath: string) : string

// Constants
const normalized
const parts
const i
const fileName
```

### File: `packages/catalog/src/lib/parsers/frontmatter.ts`

- Size: 7543 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions
- 294 lines (0 code)

**Signatures:**

```typescript
// Functions
function parseFrontmatter(
  content: string
) : ParseResult<ParsedFrontmatter<T>>
function parseFrontmatter(
  content: string
) : ParseResult<ParsedFrontmatter<T>>

// Constants
const result
const errorMessage

// Functions
function parseFrontmatterFromFile(
  filePath: string,
  readFile: (path: string) => Promise<string>
) : Promise<ParseResult<ParsedFrontmatter<T>>>
function parseFrontmatterFromFile(
  filePath: string,
  readFile: (path: string) => Promise<string>
) : Promise<ParseResult<ParsedFrontmatter<T>>>

// Constants
const content
const result
const errorMessage

// Functions
function parseAgentFrontmatter(
  content: string
) : ParseResult<ParsedFrontmatter<AgentFrontmatter>>
function parseAgentFrontmatter(
  content: string
) : ParseResult<ParsedFrontmatter<AgentFrontmatter>>

// Constants
const result
const warnings : Array<{ code: string; message: string }>

// Functions
function parseSkillFrontmatter(
  content: string
) : ParseResult<ParsedFrontmatter<SkillFrontmatter>>
function parseSkillFrontmatter(
  content: string
) : ParseResult<ParsedFrontmatter<SkillFrontmatter>>

// Constants
const result
const warnings : Array<{ code: string; message: string }>

// Functions
function hasFrontmatter(content: string) : boolean
function hasFrontmatter(content: string) : boolean

// Constants
const trimmed

// Functions
function getFrontmatterBoundaries(content: string) : {
  start: number;
  end: number;
} | null
function getFrontmatterBoundaries(content: string) : {
  start: number;
  end: number;
} | null

// Constants
const trimmed
const endMarker

// Functions
function normalizeFrontmatter(data: FrontmatterData) : FrontmatterData
function normalizeFrontmatter(data: FrontmatterData) : FrontmatterData

// Constants
const normalized : FrontmatterData
const arrayFields
const value

// Functions
function mergeFrontmatterWithDefaults(
  data: Partial<T>,
  defaults: T
) : T
function mergeFrontmatterWithDefaults(
  data: Partial<T>,
  defaults: T
) : T

// Constants
const result
```

### File: `packages/catalog/src/lib/parsers/jsdoc.ts`

- Size: 11746 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions
- 456 lines (0 code)

**Signatures:**

```typescript
// Constants
const PATTERNS

// Functions
function parseJSDoc(content: string) : ParseResult<ParsedJSDoc>
function parseJSDoc(content: string) : ParseResult<ParsedJSDoc>

// Constants
const jsdocBlocks
const headerBlock
const result
const errorMessage

// Functions
function parseJSDocBlock(block: string) : ParsedJSDoc
function parseJSDocBlock(block: string) : ParsedJSDoc

// Constants
const cleanBlock
const result : ParsedJSDoc
const processMatch
const descMatch
const lines
const firstLine
const inputsMatch
const outputsMatch
const authorMatch
const versionMatch
const sinceMatch
const deprecatedMatch
const seeMatches
const exampleMatches
const tags : Record<string, string>
const tagMatches
const knownTags
const tagName
const tagValue

// Functions
function parseInputsOutputs(
  definition: string,
  type: 'input' | 'output'
) : JSDocInput[] | JSDocOutput[]

// Constants
const clean
const items : Array<JSDocInput | JSDocOutput>
const parts
const trimmed
const colonIndex
const name
const typeStr
const required

// Functions
function splitByComma(str: string) : string[]

// Constants
const parts : string[]
const current
const depth

// Functions
function extractAllJSDocBlocks(content: string) : ParseResult<ParsedJSDoc[]>
function extractAllJSDocBlocks(content: string) : ParseResult<ParsedJSDoc[]>

// Constants
const blocks
const parsed
const errorMessage

// Functions
function findJSDocForFunction(
  content: string,
  functionName: string
) : ParseResult<ParsedJSDoc>
function findJSDocForFunction(
  content: string,
  functionName: string
) : ParseResult<ParsedJSDoc>

// Constants
const patterns
const match
const jsdocMatch
const errorMessage

// Functions
function parseParams(block: string) : JSDocInput[]
function parseParams(block: string) : JSDocInput[]

// Constants
const params : JSDocInput[]
const cleanBlock
const paramRegex
const match
const isOptional
const name
const defaultValue
const description
const typeStr

// Functions
function validateJSDoc(jsdoc: ParsedJSDoc) : {
  valid: boolean;
  missing: string[];
  warnings: string[];
}
function validateJSDoc(jsdoc: ParsedJSDoc) : {
  valid: boolean;
  missing: string[];
  warnings: string[];
}

// Constants
const missing : string[]
const warnings : string[]
```

### File: `packages/catalog/src/lib/parsers/markdown.ts`

- Size: 9897 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 15 functions
- 387 lines (0 code)

**Signatures:**

```typescript
// Constants
const PATTERNS

// Functions
function parseMarkdownSections(content: string) : ParseResult<MarkdownSection[]>
function parseMarkdownSections(content: string) : ParseResult<MarkdownSection[]>

// Constants
const lines
const sections : MarkdownSection[]
const stack : Array<{ section: MarkdownSection; level: number }>
const currentContent : string[]
const headerMatch
const lastItem
const hashMarks
const titleText
const level
const title
const newSection : MarkdownSection
const top
const parent
const lastStackItem
const errorMessage

// Functions
function cleanEmptySubsections(sections: MarkdownSection[]) : void
function findSection(
  sections: MarkdownSection[],
  title: string
) : MarkdownSection | undefined
function findSection(
  sections: MarkdownSection[],
  title: string
) : MarkdownSection | undefined

// Constants
const normalizedTitle
const found

// Functions
function findSectionsByPattern(
  sections: MarkdownSection[],
  pattern: RegExp
) : MarkdownSection[]
function findSectionsByPattern(
  sections: MarkdownSection[],
  pattern: RegExp
) : MarkdownSection[]

// Constants
const results : MarkdownSection[]

// Functions
function extractListItems(content: string) : string[]
function extractListItems(content: string) : string[]

// Constants
const items : string[]
const lines
const unorderedMatch
const item
const orderedMatch
const item

// Functions
function extractListFromSection(
  sections: MarkdownSection[],
  sectionTitle: string
) : string[]
function extractListFromSection(
  sections: MarkdownSection[],
  sectionTitle: string
) : string[]

// Constants
const section

// Functions
function getPlainText(content: string) : string
function getPlainText(content: string) : string

// Constants
const text

// Functions
function flattenSections(sections: MarkdownSection[]) : MarkdownSection[]
function flattenSections(sections: MarkdownSection[]) : MarkdownSection[]

// Constants
const result : MarkdownSection[]

// Functions
function getSectionPath(sections: MarkdownSection[], targetTitle: string) : string[]
function getSectionPath(sections: MarkdownSection[], targetTitle: string) : string[]

// Constants
const normalizedTarget

// Functions
function findPath(
    sects: MarkdownSection[],
    currentPath: string[]
  ) : string[] | null

// Constants
const newPath
const found

// Functions
function getSectionSummary(section: MarkdownSection, maxLength: number = 200) : string
function getSectionSummary(section: MarkdownSection, maxLength: number = 200) : string

// Constants
const plainText
const paragraphs
const firstParagraph
const truncated
const lastSpace

// Functions
function countWords(content: string) : number
function countWords(content: string) : number

// Constants
const plainText
const words

// Functions
function extractHeadings(content: string) : Array<{ level: number; title: string }>
function extractHeadings(content: string) : Array<{ level: number; title: string }>

// Constants
const headings : Array<{ level: number; title: string }>
const lines
const match
const hashMarks
const titleText
```

### File: `packages/catalog/src/lib/parsers/process.ts`

- Size: 11953 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 17 functions, 1 interfaces
- 483 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ProcessParserOptions
interface ProcessParserOptions

// Constants
const DEFAULT_OPTIONS : ProcessParserOptions

// Functions
function parseProcessContent(
  content: string,
  filePath: string = '',
  options: ProcessParserOptions = {}
) : ParseResult<ParsedProcess>
function parseProcessContent(
  content: string,
  filePath: string = '',
  options: ProcessParserOptions = {}
) : ParseResult<ParsedProcess>

// Constants
const opts
const warnings : Array<{ code: string; message: string }>
const jsdocResult
const jsdoc
const validation
const tasks : DefineTaskCall[]
const exports : ExportInfo[]
const hasProcess
const astResult
const processId
const category
const process : ParsedProcess

// Functions
function parseProcessFile(
  filePath: string,
  readFile: (path: string) => Promise<string>,
  options: ProcessParserOptions = {}
) : Promise<ParseResult<ParsedProcess>>
function parseProcessFile(
  filePath: string,
  readFile: (path: string) => Promise<string>,
  options: ProcessParserOptions = {}
) : Promise<ParseResult<ParsedProcess>>

// Constants
const content
const errorMessage

// Functions
function parseInputsFromRaw(raw: string | undefined) : JSDocInput[]

// Constants
const clean
const inner
const inputs : JSDocInput[]
const parts
const colonIdx
const name
const type
const required

// Functions
function parseOutputsFromRaw(raw: string | undefined) : JSDocOutput[]

// Constants
const clean
const inner
const outputs : JSDocOutput[]
const parts
const colonIdx
const name
const type

// Functions
function splitByTopLevelComma(str: string) : string[]

// Constants
const parts : string[]
const current
const depth

// Functions
function extractCategory(processId: string, filePath: string) : string

// Constants
const parts
const normalized
const methodologiesMatch
const specializationsMatch

// Functions
function getDirectory(filePath: string) : string

// Constants
const normalized
const lastSlash

// Functions
function validateProcess(process: ParsedProcess) : {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
function validateProcess(process: ParsedProcess) : {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// Constants
const errors : string[]
const warnings : string[]

// Functions
function processToCatalogEntry(process: ParsedProcess) : {
  id: string;
  type: 'process';
  name: string;
  description: string;
  path: string;
  category: string;
  tags: string[];
  metadata: Record<string, unknown>;
}
function processToCatalogEntry(process: ParsedProcess) : {
  id: string;
  type: 'process';
  name: string;
  description: string;
  path: string;
  category: string;
  tags: string[];
  metadata: Record<string, unknown>;
}

// Constants
const name
const tags : string[]

// Functions
function generateProcessSummary(process: ParsedProcess) : string
function generateProcessSummary(process: ParsedProcess) : string

// Constants
const parts : string[]
const required
const kind

// Functions
function getProcessTaskNames(process: ParsedProcess) : string[]
function getProcessTaskNames(process: ParsedProcess) : string[]
function getRequiredInputs(process: ParsedProcess) : JSDocInput[]
function getRequiredInputs(process: ParsedProcess) : JSDocInput[]
function groupProcessesByCategory(
  processes: ParsedProcess[]
) : Map<string, ParsedProcess[]>
function groupProcessesByCategory(
  processes: ParsedProcess[]
) : Map<string, ParsedProcess[]>

// Constants
const grouped
const category
```

### File: `packages/catalog/src/lib/parsers/skill.ts`

- Size: 11901 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions, 1 interfaces
- 477 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SkillParserOptions
interface SkillParserOptions

// Constants
const DEFAULT_OPTIONS : SkillParserOptions

// Functions
function parseSkillContent(
  content: string,
  filePath: string = '',
  options: SkillParserOptions = {}
) : ParseResult<ParsedSkill>
function parseSkillContent(
  content: string,
  filePath: string = '',
  options: SkillParserOptions = {}
) : ParseResult<ParsedSkill>

// Constants
const opts
const frontmatterResult
const frontmatterResult
const sections : MarkdownSection[]
const sectionsResult
const skill : ParsedSkill

// Functions
function parseSkillFile(
  filePath: string,
  readFile: (path: string) => Promise<string>,
  options: SkillParserOptions = {}
) : Promise<ParseResult<ParsedSkill>>
function parseSkillFile(
  filePath: string,
  readFile: (path: string) => Promise<string>,
  options: SkillParserOptions = {}
) : Promise<ParseResult<ParsedSkill>>

// Constants
const content
const errorMessage

// Functions
function normalizeSkillMetadata(
  metadata: SkillMetadata | undefined,
  filePath: string
) : SkillMetadata

// Constants
const normalized : SkillMetadata

// Functions
function extractPurpose(sections: MarkdownSection[]) : string | undefined

// Constants
const purposeSection
const overviewSection
const descSection

// Functions
function extractCapabilities(sections: MarkdownSection[]) : string[]

// Constants
const capabilities : string[]
const capabilitiesSection
const featuresSection
const whatSection

// Functions
function extractUsageGuidelines(sections: MarkdownSection[]) : string[]

// Constants
const guidelines : string[]
const usageSectionNames
const section
const items

// Functions
function extractTools(sections: MarkdownSection[]) : string[]

// Constants
const tools : string[]
const toolSectionNames
const section

// Functions
function getDirectory(filePath: string) : string

// Constants
const normalized
const lastSlash

// Functions
function validateSkill(skill: ParsedSkill) : {
  valid: boolean;
  errors: string[];
  warnings: string[];
}
function validateSkill(skill: ParsedSkill) : {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// Constants
const errors : string[]
const warnings : string[]

// Functions
function skillToCatalogEntry(skill: ParsedSkill) : {
  id: string;
  type: 'skill';
  name: string;
  description: string;
  path: string;
  domain?: string;
  specialization?: string;
  category?: string;
  phase?: number;
  tags: string[];
  metadata: Record<string, unknown>;
}
function skillToCatalogEntry(skill: ParsedSkill) : {
  id: string;
  type: 'skill';
  name: string;
  description: string;
  path: string;
  domain?: string;
  specialization?: string;
  category?: string;
  phase?: number;
  tags: string[];
  metadata: Record<string, unknown>;
}
function generateSkillSummary(skill: ParsedSkill) : string
function generateSkillSummary(skill: ParsedSkill) : string

// Constants
const parts : string[]

// Functions
function isSkillCompatibleWithTools(
  skill: ParsedSkill,
  availableTools: string[]
) : boolean
function isSkillCompatibleWithTools(
  skill: ParsedSkill,
  availableTools: string[]
) : boolean
function groupSkillsByCategory(
  skills: ParsedSkill[]
) : Map<string, ParsedSkill[]>
function groupSkillsByCategory(
  skills: ParsedSkill[]
) : Map<string, ParsedSkill[]>

// Constants
const grouped
const category
```

### File: `packages/catalog/src/lib/parsers/types.ts`

- Size: 9146 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 27 interfaces, 1 types
- 438 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ParseResult
interface ParseResult
interface ParseError
interface ParseError
interface ParseWarning
interface ParseWarning
interface SourceLocation
interface SourceLocation
interface FrontmatterData
interface FrontmatterData
interface ParsedFrontmatter
interface ParsedFrontmatter
interface AgentMetadata
interface AgentMetadata
interface AgentFrontmatter
interface AgentFrontmatter
interface MarkdownSection
interface MarkdownSection
interface ParsedAgent
interface ParsedAgent
interface SkillMetadata
interface SkillMetadata
interface SkillFrontmatter
interface SkillFrontmatter
interface ParsedSkill
interface ParsedSkill
interface JSDocInput
interface JSDocInput
interface JSDocOutput
interface JSDocOutput
interface ParsedJSDoc
interface ParsedJSDoc
interface DefineTaskCall
interface DefineTaskCall
interface ExportInfo
interface ExportInfo
interface ParsedAST
interface ParsedAST
interface ParsedProcess
interface ParsedProcess
interface DomainInfo
interface DomainInfo
interface SpecializationInfo
interface SpecializationInfo
interface DirectoryScanResult
interface DirectoryScanResult

// Type Aliases
type CatalogItemType

// Traits/Interfaces
interface CatalogEntry
interface CatalogEntry
interface ProcessCatalog
interface ProcessCatalog
interface ParserOptions
interface ParserOptions
interface Parser
interface Parser
```

### File: `packages/catalog/src/lib/utils.ts`

- Size: 316 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 10 lines (0 code)

**Signatures:**

```typescript
// Functions
function cn(...inputs: ClassValue[]) : string
function cn(...inputs: ClassValue[]) : string
```

### File: `packages/sdk/.eslintrc.cjs`

- Size: 626 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/sdk/scripts/smoke-cli.js`

- Size: 19598 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 31 functions
- 589 lines (0 code)

**Signatures:**

```javascript
// Constants
const fs
const fsp
const path
const require
const RUN_ID
const PROCESS_ID
const PROCESS_EXPORT
const pkgRoot
const repoRoot
const fixtureRoot
const fixtureProcessesDir
const fixtureInputsDir
const defaultRunsDir
const defaultCliEntry
const deterministicModulePath
const deterministicIndexPath

// Functions
function main(())

// Constants
const options
const runsRoot
const cliEntry
const startTime
const deterministicRequireTarget
const hookPath
const context
const summary

// Functions
function parseArgs((argv))

// Constants
const options
const i
const arg

// Functions
function expectFlagValue((argv, index, flag))

// Constants
const value

// Functions
function printHelp(())
function resolveWorkspacePath((input))

// Constants
const normalized

// Functions
function ensureBuildArtifacts((cliEntry))
function ensureFile((targetPath, message))
function resolveDeterministicModule(())

// Constants
const candidates
const exportsObject

// Functions
function prepareRunsRoot((runsRoot))
function cleanupRunsRoot((runsRoot))
function stageFixtures((runsRoot))
function ensureDirectory((targetPath, message))

// Constants
const stats

// Functions
function copyDirectory((source, destination))

// Constants
const entries
const sourcePath
const destPath

// Functions
function writeDeterministicBootstrap((runsRoot, modulePath))

// Constants
const hookDir
const hookPath
const counterPath
const payload

// Functions
function runSmokeSequence((context))

// Constants
const sequence
const runCreate
const runStatus
const runIterate
const runEvents
const runRebuild
const taskList
const effectId
const taskShowRedacted
const taskShowUnredacted
const firstValuePath
const firstPost
const runIterate2
const taskList2
const effectId2
const secondValuePath
const secondPost
const runIterate3

// Functions
function validateRunCreate((payload, context))

// Constants
const resolvedRunDir

// Functions
function ensureWaiting((payload))
function ensureCompleted((payload))
function pickEffectIdFromTaskList((payload))

// Constants
const tasks
const candidate

// Functions
function validateEventPaths((payload, runDir))

// Constants
const events
const absolute

// Functions
function ensureDryRunPlan((payload, runDir))
function validateTaskList((payload, effectId))

// Constants
const tasks
const target

// Functions
function ensurePayloadRedacted((payload))
function ensurePayloadVisible((payload, secretToken, internalKey))
function ensureDryRunStatus((payload))
function assertPosixRefs((entry))

// Constants
const refs
const value

// Functions
function runCliJson((context, args, options = {}))

// Constants
const spawnArgs
const env
const result
const trimmed

// Functions
function withRunsDir((context, args))
function buildEnv((overrides = {}))

// Constants
const env

// Functions
function exec((command, args, options))

// Constants
const child
const stdout
const stderr

// Functions
function logSummary((sequence, durationMs))
```

### File: `packages/sdk/src/cli/commands/runIterate.ts`

- Size: 6526 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions, 2 interfaces
- 219 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface RunIterateOptions
interface RunIterateOptions
interface RunIterateResult
interface RunIterateResult

// Functions
function runIterate(options: RunIterateOptions) : Promise<RunIterateResult>
function runIterate(options: RunIterateOptions) : Promise<RunIterateResult>

// Constants
const options
const metadata
const runId
const iteration
const projectRoot
const iterationResult
const completionSecret
const iterationStartPayload : JsonRecord
const hookResult
const hookDecision
const action
const reason
const count
const until
const status : RunIterateResult["status"]
const iterationEndPayload
const result : RunIterateResult

// Functions
function parseHookDecision(output: unknown) : {
  action?: string;
  reason?: string;
  count?: number;
  until?: number;
  status?: string;
}

// Constants
const record
const action
const reason
const status
const count
const until

// Functions
function parseMaybeJsonRecord(output: unknown) : JsonRecord | undefined

// Constants
const parsed
```

### File: `packages/sdk/src/cli/completionSecret.ts`

- Size: 536 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 12 lines (0 code)

**Signatures:**

```typescript
// Constants
const COMPLETION_SECRET_SALT

// Functions
function deriveCompletionSecret(runId: string) : string
function deriveCompletionSecret(runId: string) : string
function resolveCompletionSecret(metadata: RunMetadata) : string
function resolveCompletionSecret(metadata: RunMetadata) : string
```

### File: `packages/sdk/src/hooks/dispatcher.ts`

- Size: 5137 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 198 lines (0 code)

**Signatures:**

```typescript
// Functions
function findHookDispatcherPath(startCwd: string) : string | null
function findHookDispatcherPath(startCwd: string) : string | null

// Constants
const claudePluginRoot
const candidate
const current
const i
const candidate
const parent

// Functions
function callHook(
  options: HookDispatcherOptions
) : Promise<HookResult>
function callHook(
  options: HookDispatcherOptions
) : Promise<HookResult>

// Constants
const options
const dispatcherPath
const payloadJson
const startTime
const child
const stdout
const stderr
const timedOut
const timeoutHandle
const result : HookResult
const _duration
const result : HookResult
const executedHooks
const result : HookResult

// Functions
function parseHookExecutionSummary(stderr: string) : HookExecutionResult[]

// Constants
const results : HookExecutionResult[]
const lines
const match
const location

// Functions
function tryParseJson(str: string) : unknown
```

### File: `packages/sdk/src/hooks/types.ts`

- Size: 4196 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 interfaces, 3 types
- 208 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type KnownHookType
type HookType

// Traits/Interfaces
interface HookResult
interface HookResult
interface HookExecutionResult
interface HookExecutionResult
interface OnRunStartPayload
interface OnRunStartPayload
interface OnRunCompletePayload
interface OnRunCompletePayload
interface OnRunFailPayload
interface OnRunFailPayload
interface OnTaskStartPayload
interface OnTaskStartPayload
interface OnTaskCompletePayload
interface OnTaskCompletePayload
interface OnStepDispatchPayload
interface OnStepDispatchPayload
interface OnIterationStartPayload
interface OnIterationStartPayload
interface OnIterationEndPayload
interface OnIterationEndPayload
interface OnBreakpointPayload
interface OnBreakpointPayload
interface PreCommitPayload
interface PreCommitPayload
interface PreBranchPayload
interface PreBranchPayload
interface PostPlanningPayload
interface PostPlanningPayload
interface OnScorePayload
interface OnScorePayload

// Type Aliases
type HookPayload

// Traits/Interfaces
interface HookDispatcherOptions
interface HookDispatcherOptions
```

### File: `packages/sdk/src/runtime/commitEffectResult.ts`

- Size: 6993 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions
- 194 lines (0 code)

**Signatures:**

```typescript
// Functions
function commitEffectResult(options: CommitEffectResultOptions) : Promise<CommitEffectResultArtifacts>
function commitEffectResult(options: CommitEffectResultOptions) : Promise<CommitEffectResultArtifacts>

// Constants
const effectIndex
const record
const resultPayload
const stdoutRef
const stderrRef
const eventError
const resolvedEvent

// Functions
function ensureInvocationKeyMatches(options: CommitEffectResultOptions, record: EffectRecord)
function serializeEffectError(error: unknown) : SerializedEffectError
function guardResultPayload(options: CommitEffectResultOptions)
function validateResultPayload(options: CommitEffectResultOptions)
function logCommitFailure(options: CommitEffectResultOptions, reason: string, extra: Record<string, unknown> = {})
function buildResultPayload(options: CommitEffectResultOptions)

// Constants
const base

// Functions
function requireTaskId(record: EffectRecord) : string
```

### File: `packages/sdk/src/runtime/constants.ts`

- Size: 119 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const replaySchemaVersion
```

### File: `packages/sdk/src/runtime/createRun.ts`

- Size: 3772 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 114 lines (0 code)

**Signatures:**

```typescript
// Functions
function createRun(options: CreateRunOptions) : Promise<CreateRunResult>
function createRun(options: CreateRunOptions) : Promise<CreateRunResult>

// Constants
const runId
const runDir
const normalizedEntrypoint
const requestId
const providedSecret
const completionSecret
const extraMetadata
const lockAcquired
const eventPayload : Record<string, unknown>
const entryString
const projectRoot

// Functions
function validateRunId(runId: string)
function normalizeEntrypoint(runDir: string, importPath: string, exportName?: string) : RunEntrypointMetadata

// Constants
const entryImport

// Functions
function toRunRelativePosix(runDir: string, importPath: string) : string

// Constants
const relative
```

### File: `packages/sdk/src/runtime/errorUtils.ts`

- Size: 1298 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 51 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SerializedRuntimeError
interface SerializedRuntimeError

// Functions
function serializeUnknownError(error: unknown) : SerializedRuntimeError
function serializeUnknownError(error: unknown) : SerializedRuntimeError
function toSerializedEffectError(error: unknown) : SerializedEffectError
function toSerializedEffectError(error: unknown) : SerializedEffectError

// Constants
const err
const serialized
```

### File: `packages/sdk/src/runtime/exceptions.ts`

- Size: 3138 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 10 classes, 1 interfaces, 1 types
- 101 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BabysitterErrorDetails
interface BabysitterErrorDetails

// Structs/Classes
class BabysitterRuntimeError
class BabysitterRuntimeError
class BabysitterIntrinsicError
class BabysitterIntrinsicError
class EffectRequestedError
class EffectRequestedError
class EffectPendingError
class EffectPendingError
class ParallelPendingError
class ParallelPendingError
class InvocationCollisionError
class InvocationCollisionError
class RunFailedError
class RunFailedError
class MissingProcessContextError
class MissingProcessContextError
class InvalidTaskDefinitionError
class InvalidTaskDefinitionError
class InvalidSleepTargetError
class InvalidSleepTargetError

// Functions
function isIntrinsicError(error: unknown)
function isIntrinsicError(error: unknown)

// Type Aliases
type ErrorWithData

// Functions
function rehydrateSerializedError(data?: SerializedEffectError) : Error
function rehydrateSerializedError(data?: SerializedEffectError) : Error

// Constants
const name
const message
const err
```

### File: `packages/sdk/src/runtime/hooks/runtime.ts`

- Size: 2200 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 88 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface RuntimeHookOptions
interface RuntimeHookOptions

// Functions
function callRuntimeHook(
  hookType: HookType,
  payload: Record<string, unknown>,
  options: RuntimeHookOptions
) : Promise<HookResult>
function callRuntimeHook(
  hookType: HookType,
  payload: Record<string, unknown>,
  options: RuntimeHookOptions
) : Promise<HookResult>

// Constants
const options
const fullPayload
const result
const errorMessage

// Functions
function createRuntimeHookPayload(
  hookType: HookType,
  data: T
) : T & { hookType: HookType; timestamp: string }
function createRuntimeHookPayload(
  hookType: HookType,
  data: T
) : T & { hookType: HookType; timestamp: string }
```

### File: `packages/sdk/src/runtime/instrumentation.ts`

- Size: 532 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces, 1 types
- 20 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type RuntimeLogger

// Traits/Interfaces
interface MetricPayload
interface MetricPayload

// Functions
function emitRuntimeMetric(
  logger: RuntimeLogger | undefined,
  metric: string,
  payload: Record<string, unknown> = {}
) : void
function emitRuntimeMetric(
  logger: RuntimeLogger | undefined,
  metric: string,
  payload: Record<string, unknown> = {}
) : void

// Constants
const entry : MetricPayload
```

### File: `packages/sdk/src/runtime/intrinsics/breakpoint.ts`

- Size: 1489 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 54 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BreakpointArgs

// Constants
const BREAKPOINT_TASK_ID
const DEFAULT_BREAKPOINT_LABEL
const breakpointTask : DefinedTask<BreakpointArgs, void>

// Functions
function runBreakpointIntrinsic(
  payload: T,
  context: TaskIntrinsicContext,
  options?: TaskInvokeOptions
)
function runBreakpointIntrinsic(
  payload: T,
  context: TaskIntrinsicContext,
  options?: TaskInvokeOptions
)

// Constants
const label
const invokeOptions

// Functions
function deriveBreakpointLabel(payload: unknown, provided?: string) : string

// Constants
const inferred
```

### File: `packages/sdk/src/runtime/intrinsics/hook.ts`

- Size: 1978 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 interfaces
- 76 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface HookIntrinsicOptions
interface HookIntrinsicOptions

// Functions
function runHookIntrinsic(
  hookType: string,
  payload: Record<string, unknown>,
  context: InternalProcessContext,
  options?: HookIntrinsicOptions
) : Promise<HookResult>
function runHookIntrinsic(
  hookType: string,
  payload: Record<string, unknown>,
  context: InternalProcessContext,
  options?: HookIntrinsicOptions
) : Promise<HookResult>

// Constants
const fullPayload
const label
const result
const hookCount
const successCount
```

### File: `packages/sdk/src/runtime/intrinsics/orchestratorTask.ts`

- Size: 1009 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 interfaces
- 38 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface OrchestratorTaskArgs

// Constants
const ORCHESTRATOR_TASK_ID
const orchestratorTask : DefinedTask<OrchestratorTaskArgs, unknown>

// Functions
function runOrchestratorTaskIntrinsic(
  payload: TPayload,
  context: TaskIntrinsicContext,
  options?: TaskInvokeOptions
) : Promise<TResult>
function runOrchestratorTaskIntrinsic(
  payload: TPayload,
  context: TaskIntrinsicContext,
  options?: TaskInvokeOptions
) : Promise<TResult>

// Constants
const label
const invokeOptions
```

### File: `packages/sdk/src/runtime/intrinsics/parallel.ts`

- Size: 1399 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 50 lines (0 code)

**Signatures:**

```typescript
// Functions
function runParallelAll(thunks: Array<() => T | Promise<T>>) : Promise<T[]>
function runParallelAll(thunks: Array<() => T | Promise<T>>) : Promise<T[]>

// Constants
const results : T[]
const pending : EffectAction[]
const value
const actions

// Functions
function runParallelMap(
  items: TItem[],
  fn: (item: TItem) => TOut | Promise<TOut>
) : Promise<TOut[]>
function runParallelMap(
  items: TItem[],
  fn: (item: TItem) => TOut | Promise<TOut>
) : Promise<TOut[]>

// Constants
const thunks

// Functions
function dedupeEffectActions(actions: EffectAction[]) : EffectAction[]
function dedupeEffectActions(actions: EffectAction[]) : EffectAction[]
function collectPendingActions(error: unknown) : EffectAction[]
```

### File: `packages/sdk/src/runtime/intrinsics/sleep.ts`

- Size: 2050 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 interfaces
- 82 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface SleepArgs

// Constants
const SLEEP_TASK_ID
const sleepTask : DefinedTask<SleepArgs, void>

// Functions
function runSleepIntrinsic(
  target: string | number,
  context: TaskIntrinsicContext,
  options?: TaskInvokeOptions
)
function runSleepIntrinsic(
  target: string | number,
  context: TaskIntrinsicContext,
  options?: TaskInvokeOptions
)

// Constants
const epoch
const nowMs
const iso
const label
const invokeOptions

// Functions
function normalizeSleepTarget(target: string | number) : number

// Constants
const parsed

// Functions
function shouldShortCircuitPending(error: unknown, nowMs: number) : boolean

// Constants
const metadata
```

### File: `packages/sdk/src/runtime/intrinsics/task.ts`

- Size: 9318 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 13 functions, 2 interfaces
- 300 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface TaskIntrinsicContext
interface TaskIntrinsicContext
interface TaskIntrinsicInvokeOptions
interface TaskIntrinsicInvokeOptions

// Functions
function runTaskIntrinsic(
  options: TaskIntrinsicInvokeOptions<TArgs, TResult>
) : Promise<TResult>
function runTaskIntrinsic(
  options: TaskIntrinsicInvokeOptions<TArgs, TResult>
) : Promise<TResult>

// Constants
const options
const stepId
const invocation
const existing

// Functions
function handleExistingInvocation(
  record: EffectRecord,
  options: TaskIntrinsicInvokeOptions<TArgs, TResult>
) : Promise<TResult>

// Constants
const taskDef
const error
const stored : StoredTaskResult | undefined
const err
const value

// Functions
function requestNewEffect(
  stepId: string,
  invocationKey: string,
  invocationHash: string,
  options: TaskIntrinsicInvokeOptions<TArgs, TResult>
) : Promise<TResult>

// Constants
const effectId
const buildCtx
const taskDef
const kind
const normalizedLabels
const label
const labelMetadata
const eventPayload
const appendResult
const syntheticEvent : JournalEvent
const actionRecord : EffectRecord
const action

// Functions
function ensureTaskDefinition(runDir: string, record: EffectRecord) : Promise<TaskDef>

// Constants
const stored

// Functions
function buildEffectAction(record: EffectRecord, taskDef: TaskDef) : EffectAction

// Constants
const schedulerHints

// Functions
function deriveSchedulerHints(taskDef: TaskDef) : EffectSchedulerHints | undefined

// Constants
const hints : EffectSchedulerHints
const sleepHint

// Functions
function extractSleepTarget(taskDef: TaskDef) : number | undefined

// Constants
const metadataTarget

// Functions
function normalizeRef(runDir: string, ref: string)
function resolveStoredResultValue(runDir: string, stored: StoredTaskResult) : Promise<unknown>

// Constants
const absolute
const raw

// Functions
function collectInvocationLabels(ctx: TaskBuildContext, taskDef: TaskDef) : string[]

// Constants
const combined : string[]
const addLabels

// Functions
function dedupeLabels(values: string[]) : string[]

// Constants
const seen
const normalized : string[]
const trimmed

// Functions
function deriveEffectLabel(
  ctx: TaskBuildContext,
  taskDef: TaskDef,
  labels: string[],
  fallbackTaskId: string
) : string
```

### File: `packages/sdk/src/runtime/invocation/hashInvocationKey.ts`

- Size: 532 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 2 interfaces
- 19 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface InvocationKeyComponents
interface InvocationKeyComponents
interface InvocationKeyInfo
interface InvocationKeyInfo

// Functions
function hashInvocationKey(components: InvocationKeyComponents) : InvocationKeyInfo
function hashInvocationKey(components: InvocationKeyComponents) : InvocationKeyInfo

// Constants
const key
const digest
```

### File: `packages/sdk/src/runtime/orchestrateIteration.ts`

- Size: 9951 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions, 1 interfaces, 2 types
- 316 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type ProcessFunction

// Constants
const dynamicImportModule : (specifier: string) => Promise<Record<string, unknown>>

// Functions
function orchestrateIteration(options: OrchestrateOptions) : Promise<IterationResult>
function orchestrateIteration(options: OrchestrateOptions) : Promise<IterationResult>

// Constants
const iterationStartedAt
const nowFn
const engine
const defaultEntrypoint
const processFn
const inputs
const finalStatus : IterationResult["status"]
const logger
const projectRoot
const output
const outputRef
const result : IterationResult
const waiting
const failure
const result : IterationResult

// Traits/Interfaces
interface EntrypointDefaults

// Functions
function loadProcessFunction(
  options: OrchestrateOptions,
  defaults: EntrypointDefaults,
  runDir: string
) : Promise<ProcessFunction>

// Constants
const importPath
const exportName
const resolvedPath
const moduleUrl
const mod : Record<string, unknown>
const candidate

// Type Aliases
type WaitingIterationResult

// Functions
function asWaitingResult(error: unknown) : WaitingIterationResult | null
function resolveNow(now?: Date | (() => Date)) : () => Date

// Constants
const fixed

// Functions
function initializeReplayEngine(
  options: OrchestrateOptions,
  nowFn: () => Date,
  iterationStartedAt: number
) : Promise<ReplayEngine>
function annotateWaitingActions(actions: EffectAction[]) : EffectAction[]

// Constants
const pendingCount
const derivedSleep
const nextHints

// Functions
function deriveSleepHint(action: EffectAction) : number | undefined

// Constants
const direct
const metadataTarget

// Functions
function mergeSchedulerHints(
  base: EffectSchedulerHints | undefined,
  extra: EffectSchedulerHints
) : EffectSchedulerHints | undefined

// Constants
const merged : EffectSchedulerHints
const changed

// Functions
function createIterationMetadata(engine: ReplayEngine) : IterationMetadata
```

### File: `packages/sdk/src/runtime/processContext.ts`

- Size: 3167 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions, 3 interfaces
- 90 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ProcessContextInit
interface ProcessContextInit
interface InternalProcessContext
interface InternalProcessContext

// Constants
const contextStorage

// Traits/Interfaces
interface CreateProcessContextResult
interface CreateProcessContextResult

// Functions
function createProcessContext(init: ProcessContextInit) : CreateProcessContextResult
function createProcessContext(init: ProcessContextInit) : CreateProcessContextResult

// Constants
const safeLogger
const internal : InternalProcessContext
const parallelHelpers : ParallelHelpers
const processContext : ProcessContext

// Functions
function withProcessContext(internal: InternalProcessContext, fn: () => Promise<T> | T) : Promise<T>
function withProcessContext(internal: InternalProcessContext, fn: () => Promise<T> | T) : Promise<T>
function getActiveProcessContext() : InternalProcessContext | undefined
function getActiveProcessContext() : InternalProcessContext | undefined
function requireProcessContext() : InternalProcessContext
function requireProcessContext() : InternalProcessContext

// Constants
const ctx
```

### File: `packages/sdk/src/runtime/replay/createReplayEngine.ts`

- Size: 3753 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 2 interfaces
- 113 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface CreateReplayEngineOptions
interface CreateReplayEngineOptions
interface ReplayEngine
interface ReplayEngine

// Functions
function createReplayEngine(options: CreateReplayEngineOptions) : Promise<ReplayEngine>
function createReplayEngine(options: CreateReplayEngineOptions) : Promise<ReplayEngine>

// Constants
const metadata
const inputs
const effectIndex
const replayCursor
const processId
const createProcessContext

// Functions
function resolveStateCacheSnapshot({
  runDir,
  effectIndex,
}: {
  runDir: string;
  effectIndex: EffectIndex;
}) : Promise<{ snapshot: StateCacheSnapshot | null; rebuildMeta: ReplayEngine["stateRebuild"] }>

// Constants
const existingSnapshot : StateCacheSnapshot | null
const corrupted
const reason
const rebuilt
const journalHead
const rebuilt

// Functions
function ensureCompatibleLayout(layoutVersion: string | undefined, runDir: string)
```

### File: `packages/sdk/src/runtime/replay/effectIndex.ts`

- Size: 10080 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions, 1 classes, 3 interfaces, 1 types
- 288 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BuildEffectIndexOptions
interface BuildEffectIndexOptions

// Type Aliases
type SupportedEventType

// Traits/Interfaces
interface EffectRequestedPayload
interface EffectResolvedPayload

// Structs/Classes
class EffectIndex
class EffectIndex

// Constants
const events
const index
const expectedSeq
const type
const nextSeq
const shouldMatch
const payload
const effectId
const invocationKey
const stepId
const taskId
const taskDefRef
const inputsRef
const labels
const record : EffectRecord
const payload
const effectId
const record
const status : EffectStatus
const seen
const labels : string[]
const trimmed

// Functions
function loadJournalSafe(runDir: string) : Promise<JournalEvent[]>
function normalizeJournalError(error: unknown, runDir: string) : RunFailedError

// Constants
const err
const serialized

// Functions
function buildEffectIndex(options: BuildEffectIndexOptions) : Promise<EffectIndex>
function buildEffectIndex(options: BuildEffectIndexOptions) : Promise<EffectIndex>
```

### File: `packages/sdk/src/runtime/replay/replayCursor.ts`

- Size: 451 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 classes
- 23 lines (0 code)

**Signatures:**

```typescript
// Constants
const STEP_PREFIX
const STEP_WIDTH

// Structs/Classes
class ReplayCursor
class ReplayCursor

// Functions
function formatStepId(value: number)
```

### File: `packages/sdk/src/runtime/replay/stateCache.ts`

- Size: 8904 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 18 functions, 4 interfaces
- 248 lines (0 code)

**Signatures:**

```typescript
// Constants
const STATE_CACHE_SCHEMA_VERSION

// Traits/Interfaces
interface StateCacheJournalHead
interface StateCacheJournalHead
interface DerivedEffectSummary
interface DerivedEffectSummary
interface StateCacheSnapshot
interface StateCacheSnapshot
interface CreateStateCacheSnapshotOptions
interface CreateStateCacheSnapshotOptions

// Functions
function readStateCache(runDir: string) : Promise<StateCacheSnapshot | null>
function readStateCache(runDir: string) : Promise<StateCacheSnapshot | null>

// Constants
const stateFile
const raw
const parsed
const err

// Functions
function writeStateCache(runDir: string, snapshot: StateCacheSnapshot) : Promise<void>
function writeStateCache(runDir: string, snapshot: StateCacheSnapshot) : Promise<void>

// Constants
const stateDir
const stateFile

// Functions
function journalHeadsEqual(
  a?: StateCacheJournalHead | null,
  b?: StateCacheJournalHead | null
) : boolean
function journalHeadsEqual(
  a?: StateCacheJournalHead | null,
  b?: StateCacheJournalHead | null
) : boolean
function createStateCacheSnapshot(
  journalHeadOrOptions?: StateCacheJournalHead | null | CreateStateCacheSnapshotOptions
) : StateCacheSnapshot
function createStateCacheSnapshot(
  journalHeadOrOptions?: StateCacheJournalHead | null | CreateStateCacheSnapshotOptions
) : StateCacheSnapshot

// Constants
const options : CreateStateCacheSnapshotOptions

// Functions
function normalizeJournalHead(raw: unknown) : StateCacheJournalHead | null | undefined
function normalizeJournalHead(raw: unknown) : StateCacheJournalHead | null | undefined

// Constants
const seq
const ulid
const checksum

// Functions
function normalizeSnapshot(raw: unknown) : StateCacheSnapshot
function normalizeSnapshot(raw: unknown) : StateCacheSnapshot

// Constants
const journalHead
const effectsByInvocation
const pendingEffectsByKind
const savedAt
const rebuildReason
const stateVersion

// Functions
function rebuildStateCache(
  runDir: string,
  opts?: { effectIndex?: EffectIndex; reason?: string }
) : Promise<StateCacheSnapshot>
function rebuildStateCache(
  runDir: string,
  opts?: { effectIndex?: EffectIndex; reason?: string }
) : Promise<StateCacheSnapshot>

// Constants
const effectIndex
const journalHead
const stateVersion
const effectsByInvocation
const pendingEffectsByKind
const snapshot

// Functions
function deriveEffectsByInvocation(effectIndex: EffectIndex) : Record<string, DerivedEffectSummary>

// Constants
const summaries : Record<string, DerivedEffectSummary>

// Functions
function summarizeEffect(record: EffectRecord) : DerivedEffectSummary

// Constants
const summary : DerivedEffectSummary

// Functions
function derivePendingByKind(effectIndex: EffectIndex) : Record<string, number>

// Constants
const totals : Record<string, number>
const key

// Functions
function normalizeEffectSummaryMap(raw: unknown) : Record<string, DerivedEffectSummary>

// Constants
const summaries : Record<string, DerivedEffectSummary>
const summary

// Functions
function normalizeDerivedEffectSummary(raw: unknown) : DerivedEffectSummary | undefined

// Constants
const effectId
const invocationKey
const status
const summary : DerivedEffectSummary

// Functions
function normalizePendingEffects(raw: unknown) : Record<string, number>

// Constants
const totals : Record<string, number>

// Functions
function isPlainObject(value: unknown)
function isEffectStatus(value: unknown)
function isJournalHeadLike(value: unknown)
```

### File: `packages/sdk/src/runtime/types.ts`

- Size: 4166 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 interfaces, 3 types
- 161 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type ProcessLogger
type EffectStatus

// Traits/Interfaces
interface SerializedEffectError
interface SerializedEffectError
interface EffectRecord
interface EffectRecord
interface EffectSchedulerHints
interface EffectSchedulerHints
interface EffectAction
interface EffectAction
interface CreateRunOptions
interface CreateRunOptions
interface CreateRunResult
interface CreateRunResult
interface ParallelHelpers
interface ParallelHelpers
interface ProcessContext
interface ProcessContext
interface OrchestrateOptions
interface OrchestrateOptions
interface IterationMetadata
interface IterationMetadata

// Type Aliases
type IterationResult

// Traits/Interfaces
interface CommitEffectResultOptions
interface CommitEffectResultOptions
interface CommitEffectResultArtifacts
interface CommitEffectResultArtifacts
```

### File: `packages/sdk/src/storage/atomic.ts`

- Size: 1404 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 48 lines (0 code)

**Signatures:**

```typescript
// Constants
const RETRYABLE_ERRORS

// Functions
function fsyncPath(targetPath: string)

// Constants
const handle

// Functions
function writeFileAtomic(targetPath: string, data: string | Buffer, retries = 3)
function writeFileAtomic(targetPath: string, data: string | Buffer, retries = 3)

// Constants
const tempPath
const handle
const attempt
const err
```

### File: `packages/sdk/src/storage/cleanup.ts`

- Size: 2972 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 84 lines (0 code)

**Signatures:**

```typescript
// Functions
function dirSize(dir: string) : Promise<number>

// Constants
const entries
const total
const full
const stat
const error

// Functions
function getDiskUsage(runsRoot: string, runId: string) : Promise<DiskUsageReport>
function getDiskUsage(runsRoot: string, runId: string) : Promise<DiskUsageReport>

// Constants
const runDir
const journal
const total

// Functions
function findOrphanedBlobs(runsRoot: string, runId: string) : Promise<OrphanedBlobInfo[]>
function findOrphanedBlobs(runsRoot: string, runId: string) : Promise<OrphanedBlobInfo[]>

// Constants
const runDir
const blobsDir
const referenced
const tasksDir
const effects
const artifactsPath
const data
const artifacts
const error
const error
const orphaned : OrphanedBlobInfo[]
const blobs
const full
const stat
const error
```

### File: `packages/sdk/src/storage/clock.ts`

- Size: 707 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions, 1 types
- 30 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type ClockProvider

// Functions
function systemClock() : Date

// Constants
const activeClock : ClockProvider

// Functions
function getClockDate() : Date
function getClockDate() : Date

// Constants
const current

// Functions
function getClockIsoString() : string
function getClockIsoString() : string
function setClockForTests(provider: ClockProvider)
function setClockForTests(provider: ClockProvider)
function resetClock()
function resetClock()
```

### File: `packages/sdk/src/storage/createRunDir.ts`

- Size: 2367 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 69 lines (0 code)

**Signatures:**

```typescript
// Constants
const GITIGNORE_CONTENT

// Functions
function createRunDir(options: CreateRunDirOptions)
function createRunDir(options: CreateRunDirOptions)

// Constants
const runDir
const layoutVersion
const entrypoint
const createdAt
const metadata : RunMetadata

// Functions
function resolveEntrypoint(options: CreateRunDirOptions) : RunEntrypointMetadata

// Constants
const importPath
```

### File: `packages/sdk/src/storage/journal.ts`

- Size: 3643 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions, 1 interfaces
- 105 lines (0 code)

**Signatures:**

```typescript
// Functions
function formatSeq(seq: number)
function getExistingSeqs(journalDir: string)

// Constants
const entries
const err

// Functions
function appendEvent(opts: AppendEventOptions) : Promise<AppendEventResult>
function appendEvent(opts: AppendEventOptions) : Promise<AppendEventResult>

// Constants
const journalDir
const seqs
const seq
const ulid
const filename
const recordedAt
const eventPayload : JsonRecord
const contents
const checksum
const payloadWithChecksum
const targetPath

// Functions
function parseJournalFilename(filename: string)

// Constants
const seqPart
const seq

// Functions
function loadJournal(runDir: string) : Promise<JournalEvent[]>
function loadJournal(runDir: string) : Promise<JournalEvent[]>

// Constants
const journalDir
const entries
const sorted
const events : JournalEvent[]
const parseJournalFilename
const fullPath
const raw
const err

// Traits/Interfaces
interface ParsedJournalFile

// Functions
function parseJournalFile(fullPath: string) : Promise<ParsedJournalFile>

// Constants
const contents
const parseError
```

### File: `packages/sdk/src/storage/lock.ts`

- Size: 2446 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions, 1 interfaces
- 81 lines (0 code)

**Signatures:**

```typescript
// Functions
function acquireRunLock(runDir: string, owner: string) : Promise<RunLockInfo>
function acquireRunLock(runDir: string, owner: string) : Promise<RunLockInfo>

// Constants
const lockPath
const lockInfo : RunLockInfo
const err
const existing

// Functions
function releaseRunLock(runDir: string)
function releaseRunLock(runDir: string)

// Constants
const lockPath

// Functions
function readRunLock(runDir: string) : Promise<RunLockInfo | null>
function readRunLock(runDir: string) : Promise<RunLockInfo | null>

// Constants
const lockPath
const data
const err

// Functions
function isLockHeldError(error: unknown) : boolean
function sleep(ms: number)

// Traits/Interfaces
interface WithRunLockOptions
interface WithRunLockOptions

// Functions
function withRunLock(
  runDir: string,
  owner: string,
  fn: () => Promise<T>,
  options: WithRunLockOptions = {}
) : Promise<T>
function withRunLock(
  runDir: string,
  owner: string,
  fn: () => Promise<T>,
  options: WithRunLockOptions = {}
) : Promise<T>

// Constants
const retries
const delayMs
const acquired
const attempt
```

### File: `packages/sdk/src/storage/paths.ts`

- Size: 1178 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions
- 42 lines (0 code)

**Signatures:**

```typescript
// Constants
const JOURNAL_DIR
const TASKS_DIR
const BLOBS_DIR
const STATE_DIR
const ORPHANED_DIR
const PROCESS_DIR
const RUN_METADATA_FILE
const INPUTS_FILE
const LOCK_FILE
const STATE_FILE
const DEFAULT_LAYOUT_VERSION

// Functions
function getRunDir(runsRoot: string, runId: string) : string
function getRunDir(runsRoot: string, runId: string) : string
function getJournalDir(runDir: string) : string
function getJournalDir(runDir: string) : string
function getTasksDir(runDir: string) : string
function getTasksDir(runDir: string) : string
function getBlobsDir(runDir: string) : string
function getBlobsDir(runDir: string) : string
function getStateDir(runDir: string) : string
function getStateDir(runDir: string) : string
function getStateFile(runDir: string) : string
function getStateFile(runDir: string) : string
function getLockPath(runDir: string) : string
function getLockPath(runDir: string) : string
```

### File: `packages/sdk/src/storage/runFiles.ts`

- Size: 1239 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 35 lines (0 code)

**Signatures:**

```typescript
// Constants
const OUTPUT_FILE

// Functions
function readRunMetadata(runDir: string) : Promise<RunMetadata>
function readRunMetadata(runDir: string) : Promise<RunMetadata>

// Constants
const metadataPath
const raw

// Functions
function readRunInputs(runDir: string) : Promise<unknown>
function readRunInputs(runDir: string) : Promise<unknown>

// Constants
const inputsPath
const raw
const err

// Functions
function writeRunOutput(runDir: string, output: unknown)
function writeRunOutput(runDir: string, output: unknown)

// Constants
const stateDir
const outputPath
```

### File: `packages/sdk/src/storage/snapshotState.ts`

- Size: 494 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 14 lines (0 code)

**Signatures:**

```typescript
// Functions
function snapshotState(options: SnapshotStateOptions)
function snapshotState(options: SnapshotStateOptions)

// Constants
const payload
```

### File: `packages/sdk/src/storage/storeTaskArtifacts.ts`

- Size: 2247 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 56 lines (0 code)

**Signatures:**

```typescript
// Constants
const ARTIFACT_SPILL_THRESHOLD

// Functions
function writeArtifact(targetPath: string, data: Buffer | string)
function hashBuffer(data: Buffer | string)

// Constants
const buf

// Functions
function storeTaskArtifacts(options: StoreTaskArtifactsOptions)
function storeTaskArtifacts(options: StoreTaskArtifactsOptions)

// Constants
const taskDir
const savedArtifacts : Array<{ name: string; storedAt: string }>
const data
const hash
const blobPath
const artifactPath
const filePath
```

### File: `packages/sdk/src/storage/tasks.ts`

- Size: 3216 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 1 interfaces
- 98 lines (0 code)

**Signatures:**

```typescript
// Functions
function resolveTaskPath(runDir: string, effectId: string, relative: string)
function writeTaskDefinition(runDir: string, effectId: string, taskDef: JsonRecord)
function writeTaskDefinition(runDir: string, effectId: string, taskDef: JsonRecord)

// Constants
const taskDir
const taskPath

// Functions
function readTaskDefinition(runDir: string, effectId: string) : Promise<JsonRecord | undefined>
function readTaskDefinition(runDir: string, effectId: string) : Promise<JsonRecord | undefined>

// Constants
const taskPath
const raw
const err

// Functions
function readTaskResult(
  runDir: string,
  effectId: string,
  resultRef?: string
) : Promise<StoredTaskResult | undefined>
function readTaskResult(
  runDir: string,
  effectId: string,
  resultRef?: string
) : Promise<StoredTaskResult | undefined>

// Constants
const resolvedPath
const raw
const err

// Traits/Interfaces
interface WriteTaskResultOptions
interface WriteTaskResultOptions

// Functions
function writeTaskResult(options: WriteTaskResultOptions)
function writeTaskResult(options: WriteTaskResultOptions)

// Constants
const taskDir
const resultPath
const relativeResultPath
const stdoutRef
const stderrRef

// Functions
function writeTextIfProvided(
  runDir: string,
  effectId: string,
  filename: string,
  contents?: string
) : Promise<string | undefined>

// Constants
const filePath
```

### File: `packages/sdk/src/storage/types.ts`

- Size: 2352 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 interfaces, 2 types
- 123 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type JsonRecord

// Traits/Interfaces
interface RunEntrypointMetadata
interface RunEntrypointMetadata
interface RunMetadata
interface RunMetadata
interface CreateRunDirOptions
interface CreateRunDirOptions
interface AppendEventOptions
interface AppendEventOptions
interface AppendEventResult
interface AppendEventResult
interface SnapshotStateOptions
interface SnapshotStateOptions
interface StoreTaskArtifactsOptions
interface StoreTaskArtifactsOptions
interface DiskUsageReport
interface DiskUsageReport
interface OrphanedBlobInfo
interface OrphanedBlobInfo
interface RunLockInfo
interface RunLockInfo

// Type Aliases
type FileStatGetter

// Traits/Interfaces
interface JournalEvent
interface JournalEvent
interface StoredTaskResult
interface StoredTaskResult
```

### File: `packages/sdk/src/storage/ulids.ts`

- Size: 618 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 types
- 26 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type UlidFactory

// Constants
const activeFactory : UlidFactory

// Functions
function nextUlid()
function nextUlid()
function setUlidFactoryForTests(factory: UlidFactory)
function setUlidFactoryForTests(factory: UlidFactory)
function resetUlidFactory()
function resetUlidFactory()
```

### File: `packages/sdk/src/tasks/batching.ts`

- Size: 2475 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions, 3 interfaces
- 96 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface BatchedEffectSummary
interface BatchedEffectSummary
interface ParallelBatch
interface ParallelBatch
interface ParallelPendingPayload
interface ParallelPendingPayload

// Functions
function buildParallelBatch(actions: EffectAction[]) : ParallelBatch
function buildParallelBatch(actions: EffectAction[]) : ParallelBatch

// Constants
const seen
const deduped : EffectAction[]
const annotated
const summaries

// Functions
function summarizeEffectAction(action: EffectAction) : BatchedEffectSummary
function summarizeEffectAction(action: EffectAction) : BatchedEffectSummary
function toParallelPendingPayload(batch: ParallelBatch) : ParallelPendingPayload
function toParallelPendingPayload(batch: ParallelBatch) : ParallelPendingPayload
function assignParallelGroupHints(actions: EffectAction[]) : EffectAction[]

// Constants
const hash
const parallelGroupId

// Functions
function mergeSchedulerHints(
  base: EffectSchedulerHints | undefined,
  extra: EffectSchedulerHints
) : EffectSchedulerHints
```

### File: `packages/sdk/src/tasks/context.ts`

- Size: 7042 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 13 functions, 2 interfaces
- 201 lines (0 code)

**Signatures:**

```typescript
// Constants
const DEFAULT_TEXT_ENCODING : BufferEncoding
const DEFAULT_JSON_EXTENSION
const MAX_BLOB_BASENAME_LENGTH
const HASH_SUFFIX_LENGTH

// Traits/Interfaces
interface CreateTaskBuildContextOptions
interface CreateTaskBuildContextOptions

// Functions
function createTaskBuildContext(options: CreateTaskBuildContextOptions) : TaskBuildContext
function createTaskBuildContext(options: CreateTaskBuildContextOptions) : TaskBuildContext

// Constants
const runId
const runDir
const effectId
const invocationKey
const taskId
const tasksRoot
const taskDir
const normalizedLabel
const labels
const ctx : TaskBuildContext
const prepared
const blobName
const blobDir
const blobPath
const normalized
const absolute

// Functions
function mustBeNonEmpty(value: string | undefined, field: string) : string
function normalizeLabel(label?: string) : string | undefined

// Constants
const trimmed

// Functions
function buildBlobFileName(rawName: string, defaultExtension: string | undefined, contents: Buffer) : string

// Constants
const safeName
const digest

// Functions
function sanitizeBlobName(rawName: string, defaultExtension?: string) : string

// Constants
const trimmed
const normalized
const segments
const safe
const ext

// Functions
function clampBlobName(name: string) : string

// Constants
const ext
const base
const available
const truncated
const suffix

// Functions
function appendDigestSuffix(name: string, digest: string) : string

// Constants
const ext

// Functions
function normalizeTaskRelativePath(value: string) : string

// Constants
const trimmed
const replaced
const segments
const normalized : string[]

// Functions
function toRunRelative(runDir: string, absolutePath: string) : string

// Constants
const relative

// Functions
function hashString(value: string) : string
function hashBuffer(contents: Buffer) : string

// Traits/Interfaces
interface PreparedBlob

// Functions
function prepareBlobContents(value: unknown, options?: BlobWriteOptions) : PreparedBlob

// Constants
const treatAsJson
const encoding
const normalizedValue
const serialized
const withNewline

// Functions
function shouldSerializeAsJson(value: unknown, options?: BlobWriteOptions) : boolean
```

### File: `packages/sdk/src/tasks/defineTask.ts`

- Size: 2216 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions, 1 interfaces
- 72 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface DefineTaskOptions
interface DefineTaskOptions

// Functions
function defineTask(
  id: string,
  impl: TaskImpl<TArgs, TResult>,
  options: DefineTaskOptions = {}
) : DefinedTask<TArgs, TResult>
function defineTask(
  id: string,
  impl: TaskImpl<TArgs, TResult>,
  options: DefineTaskOptions = {}
) : DefinedTask<TArgs, TResult>

// Constants
const taskId
const defined : DefinedTask<TArgs, TResult>
const taskDef
const normalized
const mergedLabels

// Functions
function registerTaskId(taskId: string, options: DefineTaskOptions)
function normalizeTaskId(id: string) : string
function normalizeTaskDef(taskDef: TaskDef) : TaskDef

// Constants
const labels
```

### File: `packages/sdk/src/tasks/registry.ts`

- Size: 5695 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions, 2 classes, 3 interfaces, 1 types
- 187 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type TaskStatus

// Traits/Interfaces
interface RegisteredTaskDefinition
interface RegisteredTaskDefinition
interface RegistryEffectRecord
interface RegistryEffectRecord
interface PendingFilter
interface PendingFilter

// Structs/Classes
class DuplicateTaskIdError
class DuplicateTaskIdError
class TaskRegistry
class TaskRegistry

// Constants
const normalized : RegisteredTaskDefinition
const current
const normalizedLabels
const normalized
const current
const next : RegistryEffectRecord
const pending
const asArray
const values
const metadataLabels
const definitionLabels
const labelSet
const labels
const globalTaskRegistry

// Functions
function resetGlobalTaskRegistry() : void
function resetGlobalTaskRegistry() : void
function normalizeLabels(labels: string[]) : string[]

// Constants
const seen
const result : string[]
const trimmed

// Functions
function normalizeLabelValue(label?: string) : string | undefined

// Constants
const trimmed
```

### File: `packages/sdk/src/tasks/serializer.ts`

- Size: 8183 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 functions, 6 interfaces, 1 types
- 271 lines (0 code)

**Signatures:**

```typescript
// Constants
const TASK_SCHEMA_VERSION
const RESULT_SCHEMA_VERSION
const BLOB_THRESHOLD_BYTES

// Traits/Interfaces
interface SerializeTaskDefinitionOptions
interface SerializeTaskDefinitionOptions
interface SerializeTaskResultOptions
interface SerializeTaskResultOptions
interface TaskResultPayload
interface TaskResultPayload

// Type Aliases
type SerializedErrorPayload

// Traits/Interfaces
interface SerializeTaskDefinitionResult
interface SerializeTaskDefinitionResult
interface SerializeTaskResultOutput
interface SerializeTaskResultOutput
interface SerializedTaskDefinition
interface SerializedTaskDefinition

// Functions
function serializeAndWriteTaskDefinition(
  options: SerializeTaskDefinitionOptions
) : Promise<SerializeTaskDefinitionResult>
function serializeAndWriteTaskDefinition(
  options: SerializeTaskDefinitionOptions
) : Promise<SerializeTaskDefinitionResult>

// Constants
const serialized
const taskRef

// Functions
function serializeTaskDefinition(
  options: SerializeTaskDefinitionOptions
) : Promise<SerializedTaskDefinition>
function serializeTaskDefinition(
  options: SerializeTaskDefinitionOptions
) : Promise<SerializedTaskDefinition>

// Constants
const normalized
const serialized : SerializedTaskDefinition
const spill

// Functions
function serializeAndWriteTaskResult(
  options: SerializeTaskResultOptions
) : Promise<SerializeTaskResultOutput>
function serializeAndWriteTaskResult(
  options: SerializeTaskResultOptions
) : Promise<SerializeTaskResultOutput>

// Constants
const serialized

// Functions
function serializeTaskResult(
  options: SerializeTaskResultOptions
) : Promise<StoredTaskResult>
function serializeTaskResult(
  options: SerializeTaskResultOptions
) : Promise<StoredTaskResult>

// Constants
const serialized : StoredTaskResult
const spill

// Functions
function normalizeTaskDef(task: TaskDef)
function normalizeLabels(labels?: string[])

// Constants
const trimmed
const unique

// Functions
function normalizeJson(value: unknown) : JsonRecord | undefined
function maybeSpillLargeValue(options: {
  runDir: string;
  effectId: string;
  name: string;
  value: unknown;
}) : Promise<{ value?: unknown; ref?: string }>

// Constants
const clone
const json
const blobDir
const hash
const blobPath

// Functions
function toRunRelative(runDir: string, absolutePath: string) : string

// Constants
const relative

// Functions
function stableClone(value: T) : T

// Constants
const entries
const clone : JsonRecord
const child

// Functions
function normalizeError(error: SerializedErrorPayload) : SerializedErrorPayload
```

### File: `packages/sdk/src/tasks/types.ts`

- Size: 5300 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 16 interfaces, 5 types
- 174 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type KnownTaskKind
type TaskKind

// Traits/Interfaces
interface TaskIOHints
interface TaskIOHints
interface NodeTaskOptions
interface NodeTaskOptions
interface BreakpointTaskOptions
interface BreakpointTaskOptions
interface OrchestratorTaskOptions
interface OrchestratorTaskOptions
interface SleepTaskOptions
interface SleepTaskOptions
interface TaskDef
interface TaskDef
interface BlobWriteOptions
interface BlobWriteOptions
interface TaskBuildContext
interface TaskBuildContext

// Type Aliases
type TaskValueFactory
type TaskValueOrFactory
type TaskImpl

// Traits/Interfaces
interface DefinedTask
interface DefinedTask
interface TaskInvokeOptions
interface TaskInvokeOptions
interface TaskSerializerContext
interface TaskSerializerContext
interface NodeTaskDefinitionOptions
interface NodeTaskDefinitionOptions
interface BreakpointTaskDefinitionOptions
interface BreakpointTaskDefinitionOptions
interface OrchestratorTaskDefinitionOptions
interface OrchestratorTaskDefinitionOptions
interface SleepTaskBuilderArgs
interface SleepTaskBuilderArgs
interface SleepTaskDefinitionOptions
interface SleepTaskDefinitionOptions
```

### File: `packages/sdk/src/testing/deterministic.ts`

- Size: 15406 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 42 functions, 13 interfaces, 3 types
- 584 lines (0 code)

**Signatures:**

```typescript
// Constants
const CROCKFORD_BASE32
const DEFAULT_CLOCK_START_MS
const DEFAULT_CLOCK_STEP_MS
const TEMP_RUN_PREFIX

// Type Aliases
type MaybePromise
type ClockValue
type ClockSequenceInput

// Constants
const DEFAULT_DETERMINISTIC_RUN_ID

// Traits/Interfaces
interface SnapshotJournalEntry
interface SnapshotJournalEntry
interface SnapshotStateSummary
interface SnapshotStateSummary
interface DeterministicRunSnapshot
interface DeterministicRunSnapshot
interface TempDeterministicRun
interface TempDeterministicRun
interface TempDeterministicRunOptions
interface TempDeterministicRunOptions
interface FixedClockOptions
interface FixedClockOptions
interface FixedClockHandle
interface FixedClockHandle
interface DeterministicUlidOptions
interface DeterministicUlidOptions
interface DeterministicUlidHandle
interface DeterministicUlidHandle
interface DeterministicRunHarnessOptions
interface DeterministicRunHarnessOptions
interface DeterministicRunHarness
interface DeterministicRunHarness
interface ClockController
interface UlidController

// Functions
function withDeterministicIds(
  sequence: readonly string[],
  fn: () => MaybePromise<T>
) : Promise<T>
function withDeterministicIds(
  sequence: readonly string[],
  fn: () => MaybePromise<T>
) : Promise<T>

// Constants
const handle

// Functions
function withFixedClock(
  sequenceOrValue: ClockSequenceInput,
  fn: () => MaybePromise<T>
) : Promise<T>
function withFixedClock(
  sequenceOrValue: ClockSequenceInput,
  fn: () => MaybePromise<T>
) : Promise<T>

// Constants
const handle

// Functions
function createTempDeterministicRun(
  options: TempDeterministicRunOptions
) : Promise<TempDeterministicRun>
function createTempDeterministicRun(
  options: TempDeterministicRunOptions
) : Promise<TempDeterministicRun>

// Constants
const runsRoot
const runId
const request
const clock
const ulids
const restored

// Functions
function restoreProviders()
function cleanupRoot()

// Constants
const processPath

// Functions
function snapshotRunState(runDir: string) : Promise<DeterministicRunSnapshot>
function snapshotRunState(runDir: string) : Promise<DeterministicRunSnapshot>

// Constants
const events
const journal : SnapshotJournalEntry[]
const state
const stateSummary : SnapshotStateSummary | null

// Functions
function installFixedClock(options?: FixedClockOptions) : FixedClockHandle
function installFixedClock(options?: FixedClockOptions) : FixedClockHandle

// Constants
const controller
const depth
const release : (() => void) | null

// Functions
function apply()
function restore()
function installDeterministicUlids(options?: DeterministicUlidOptions) : DeterministicUlidHandle
function installDeterministicUlids(options?: DeterministicUlidOptions) : DeterministicUlidHandle

// Constants
const issued : string[]
const controller
const depth
const release : (() => void) | null

// Functions
function next() : string

// Constants
const value

// Functions
function apply()
function restore()
function createDeterministicRunHarness(
  options: DeterministicRunHarnessOptions
) : Promise<DeterministicRunHarness>
function createDeterministicRunHarness(
  options: DeterministicRunHarnessOptions
) : Promise<DeterministicRunHarness>

// Constants
const runsRoot
const runId
const request
const clock
const ulids
const cleanedUp

// Functions
function cleanupRoot()

// Constants
const processPath
const entrypoint

// Functions
function createClockController(options?: FixedClockOptions) : ClockController

// Constants
const sequence
const start
const stepMs

// Functions
function createSequenceClockController(sequence: readonly Date[]) : ClockController

// Constants
const index

// Functions
function currentIndex()
function clone(date: Date)

// Constants
const value

// Functions
function createTickingClockController(startMs: number, stepMs: number) : ClockController

// Constants
const initial
const current
const value

// Functions
function applyClockController(controller: ClockController) : () => void
function createUlidController(options?: DeterministicUlidOptions) : UlidController

// Constants
const epochMs
const incrementMs
const randomnessSeed

// Functions
function createPresetUlidController(preset: readonly string[]) : UlidController

// Constants
const index
const value

// Functions
function createRollingUlidController(epochMs: number, incrementMs: number, randomnessSeed: number) : UlidController

// Constants
const tick
const timestamp
const timePart
const randomPart

// Functions
function writeProcessFixture(runsRoot: string, runId: string, source: string) : Promise<string>

// Constants
const processDir
const processPath

// Functions
function normalizeClockSequence(input: ClockSequenceInput) : Date[]
function toClockDate(value: ClockValue) : Date

// Constants
const parsed

// Functions
function resolveClockStart(start?: ClockValue)
function encodeBase32(value: number, length: number)

// Constants
const remaining
const out
const idx
```

### File: `packages/sdk/src/testing/runHarness.ts`

- Size: 6821 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions, 7 interfaces, 2 types
- 243 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface FakeActionSuccess
interface FakeActionSuccess
interface FakeActionError
interface FakeActionError

// Type Aliases
type FakeActionResolution
type FakeActionResolver

// Traits/Interfaces
interface RunToCompletionWithFakeRunnerOptions
interface RunToCompletionWithFakeRunnerOptions
interface ExecutedFakeAction
interface ExecutedFakeAction
interface RunToCompletionResult
interface RunToCompletionResult
interface HarnessIterationLogEntry
interface HarnessIterationLogEntry
interface HarnessActionSnapshot
interface HarnessActionSnapshot

// Constants
const DEFAULT_MAX_ITERATIONS

// Functions
function runToCompletionWithFakeRunner(
  options: RunToCompletionWithFakeRunnerOptions
) : Promise<RunToCompletionResult>
function runToCompletionWithFakeRunner(
  options: RunToCompletionWithFakeRunnerOptions
) : Promise<RunToCompletionResult>

// Constants
const options
const maxIterations
const cleanupProviders
const executed : ExecutedFakeAction[]
const executionLog : HarnessIterationLogEntry[]
const iterations
const clockHandle
const deterministicNow
const iteration
const iterationLog : HarnessIterationLogEntry
const pendingActions
const executedThisIteration : ExecutedFakeAction[]
const handled
const resolution
const entry

// Functions
function toCommitResult(resolution: FakeActionResolution)

// Constants
const base

// Functions
function snapshotAction(action: EffectAction) : HarnessActionSnapshot
function resolveNowOption(now?: Date | (() => Date))

// Constants
const fixed

// Functions
function applyDeterministicProviders(clock?: FixedClockHandle, ulids?: DeterministicUlidHandle)

// Constants
const restoreFns : Array<() => void>
const restore
```

### File: `packages/sdk/src/testing/snapshots.ts`

- Size: 1075 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 2 interfaces
- 36 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface JournalSnapshotEntry
interface JournalSnapshotEntry
interface RunSnapshot
interface RunSnapshot

// Functions
function readJournalSnapshot(runDir: string) : Promise<JournalSnapshotEntry[]>
function readJournalSnapshot(runDir: string) : Promise<JournalSnapshotEntry[]>

// Constants
const events

// Functions
function readStateSnapshot(runDir: string) : Promise<StateCacheSnapshot | null>
function readStateSnapshot(runDir: string) : Promise<StateCacheSnapshot | null>
function captureRunSnapshot(runDir: string) : Promise<RunSnapshot>
function captureRunSnapshot(runDir: string) : Promise<RunSnapshot>

// Constants
const journal
```

### File: `packages/sdk/test-fixtures/cli/processes/smoke-node-runner.mjs`

- Size: 1337 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 48 lines (0 code)

**Signatures:**

```javascript
// Functions
function ensureFile((targetPath))
function parsePayload(())

// Constants
const flagIndex
const raw

// Functions
function writeJson((targetPath, payload))

// Constants
const outputPath
const summary
const message
```

### File: `packages/sdk/test-fixtures/cli/processes/smoke-process.mjs`

- Size: 2873 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions
- 104 lines (0 code)

**Signatures:**

```javascript
// Constants
const TASK_ID
const DEFAULT_ENTRY
const DEFAULT_SECRET_ENV
const smokeNodeTask
const entry
const payload
const secretEnv
const metadata

// Functions
function process((inputs = {}, ctx))
function process((inputs = {}, ctx))

// Constants
const taskInputs
const first
const second

// Functions
function resolveEntry((value))
function resolvePayload((payload, requestId))
function normalizeSecrets((value))

// Constants
const normalized
const provided

// Functions
function digestPayload((payload))

// Constants
const serialized
```

### File: `packages/sdk/test-fixtures/kinds/index.d.ts`

- Size: 1176 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const nodeKindFixtures : {
    id: string;
    args: {
        target: string;
        cacheKey: string;
    };
    helperLabels: string[];
    metadata: {
        subsystem: string;
    };
    env: {
        sample: {
            NODE_AUTH_TOKEN: string;
            CI_SECRET: string;
            GITHUB_TOKEN: string;
            DB_PASSWORD: string;
            PUBLIC_URL: string;
            LOG_LEVEL: string;
        };
        expectedSafe: {
            PUBLIC_URL: string;
            LOG_LEVEL: string;
        };
        expectedRedacted: string[];
    };
}
const breakpointKindFixtures : {
    id: string;
    payload: {
        reason: string;
        branch: string;
    };
    metadata: {
        severity: string;
    };
}
const orchestratorKindFixtures : {
    id: string;
    payload: {
        op: string;
        stage: string;
    };
    metadata: {
        iteration: number;
    };
    resumeCommand: string;
}
const sleepKindFixtures : {
    id: string;
    args: {
        iso: string;
        targetEpochMs: number;
    };
    helperLabels: string[];
}
```

### File: `packages/sdk/test-fixtures/runner/copy-inputs-to-output.js`

- Size: 1190 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 40 lines (0 code)

**Signatures:**

```javascript
// Constants
const fs
const path

// Functions
function readJsonIfExists((filePath))

// Constants
const raw

// Functions
function main(())

// Constants
const inputPath
const outputPath
const stdoutPath
const stderrPath
const payload
```

### File: `packages/sdk/test-fixtures/runner/emit-logs.js`

- Size: 987 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 35 lines (0 code)

**Signatures:**

```javascript
// Constants
const fs
const path

// Functions
function main(())

// Constants
const outputPath
const stdoutMessages
const stderrMessages
const payload
```

### File: `packages/sdk/test-fixtures/runner/print-env.js`

- Size: 716 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 27 lines (0 code)

**Signatures:**

```javascript
// Constants
const fs
const path

// Functions
function main(())

// Constants
const outputPath
const payload
```

### File: `packages/sdk/test-fixtures/runner/slow-logger.js`

- Size: 357 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 15 lines (0 code)

**Signatures:**

```javascript
// Constants
const count
const intervalMs

// Functions
function emitTick(())

// Constants
const stdoutLine
const stderrLine
```

### File: `packages/sdk/vitest.config.ts`

- Size: 200 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/vscode-extension/.eslintrc.cjs`

- Size: 694 bytes
- Modified: 2026-02-15 10:31:49 UTC


### File: `packages/vscode-extension/scripts/run-vsce.cjs`

- Size: 1667 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```javascript
// Constants
const cp
const fs
const os
const path
const require
const pkgDir
const vsceBin
const args
const extensionRoot
const tempRoot
const name
const manifestPath
const manifest
const outFlagIndex
const outPath
```

### File: `packages/vscode-extension/src/core/awaitingInput.ts`

- Size: 5409 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions, 2 types
- 178 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type AwaitingInputSource
type AwaitingInputStatus

// Functions
function isRecord(value: unknown)
function firstNonEmptyString(...values: unknown[]) : string | undefined
function coerceBoolean(value: unknown) : boolean | undefined

// Constants
const v

// Functions
function detectAwaitingInputFromState(state?: StateJson) : AwaitingInputStatus | undefined
function detectAwaitingInputFromState(state?: StateJson) : AwaitingInputStatus | undefined

// Constants
const active
const candidateFlags : unknown[]
const awaiting
const prompt

// Functions
function isInputRelatedToken(token: string) : boolean

// Constants
const t

// Functions
function extractPromptFromJournalEntry(entry: Record<string, unknown>) : string | undefined

// Constants
const data

// Functions
function detectAwaitingInputFromJournal(
  entries: readonly unknown[],
) : AwaitingInputStatus | undefined
function detectAwaitingInputFromJournal(
  entries: readonly unknown[],
) : AwaitingInputStatus | undefined

// Constants
const i
const raw
const entry
const typeToken
const eventToken
const statusToken
const tokens
const prompt

// Functions
function endsWithNewline(text: string) : boolean
function isLikelyInteractivePrompt(line: string) : boolean

// Constants
const trimmed
const lower

// Functions
function detectAwaitingInputFromProcessOutput(
  output: string,
) : AwaitingInputStatus | undefined
function detectAwaitingInputFromProcessOutput(
  output: string,
) : AwaitingInputStatus | undefined

// Constants
const tail
const lines
const lastLine
const prompt
```

### File: `packages/vscode-extension/src/core/breakpointClient.ts`

- Size: 3501 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions, 8 types
- 137 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type BreakpointFile
type BreakpointContext
type BreakpointPayload
type CreateBreakpointOptions
type BreakpointFeedback
type BreakpointStatus
type BreakpointDetails
type CreateBreakpointResult

// Functions
function httpJson(method: string, url: string, body?: unknown) : Promise<T>

// Constants
const headers : Record<string, string>
const fetchOptions : RequestInit
const response
const errorBody

// Functions
function createBreakpoint(
  options: CreateBreakpointOptions,
) : Promise<CreateBreakpointResult>
function createBreakpoint(
  options: CreateBreakpointOptions,
) : Promise<CreateBreakpointResult>

// Constants
const body

// Functions
function getBreakpointStatus(
  apiUrl: string,
  breakpointId: string,
) : Promise<{ status: BreakpointStatus }>
function getBreakpointStatus(
  apiUrl: string,
  breakpointId: string,
) : Promise<{ status: BreakpointStatus }>
function getBreakpointDetails(
  apiUrl: string,
  breakpointId: string,
) : Promise<BreakpointDetails>
function getBreakpointDetails(
  apiUrl: string,
  breakpointId: string,
) : Promise<BreakpointDetails>
function waitForBreakpointRelease(
  apiUrl: string,
  breakpointId: string,
  options: { intervalMs?: number; timeoutMs?: number } = {},
) : Promise<BreakpointDetails>
function waitForBreakpointRelease(
  apiUrl: string,
  breakpointId: string,
  options: { intervalMs?: number; timeoutMs?: number } = {},
) : Promise<BreakpointDetails>

// Constants
const intervalMs
const timeoutMs
const startTime
const statusResult
```

### File: `packages/vscode-extension/src/core/config.ts`

- Size: 3848 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions, 6 types
- 136 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type BabysitterResolvedSdkBinary
type BabysitterRawSettings
type ConfigIssue
type BabysitterResolvedConfig
type ResolveConfigOptions
type ResolveConfigResult

// Functions
function isNonEmptyString(value: unknown)
function resolveRunsRootPath(
  runsRoot: string,
  workspaceRoot: string | undefined,
) : string | undefined

// Constants
const trimmed

// Functions
function validateRunsRootDirectory(runsRootPath: string) : ConfigIssue | undefined

// Constants
const stat

// Functions
function resolveSdkBinaryPath(
  settings: BabysitterRawSettings,
) : BabysitterResolvedSdkBinary | undefined
function resolveBabysitterConfig(options: ResolveConfigOptions) : ResolveConfigResult
function resolveBabysitterConfig(options: ResolveConfigOptions) : ResolveConfigResult

// Constants
const issues : ConfigIssue[]
const settings
const sdkBinary
const resolvedConfig : BabysitterResolvedConfig
const rawRunsRoot
const resolvedRunsRoot
const runsRootIssue
const runsRootSource : NonNullable<BabysitterResolvedConfig['runsRoot']>['source']
```

### File: `packages/vscode-extension/src/core/copyFileContents.ts`

- Size: 3858 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 types
- 123 lines (0 code)

**Signatures:**

```typescript
// Constants
const DEFAULT_MAX_COPY_CONTENTS_BYTES
const DEFAULT_BINARY_SNIFF_BYTES

// Type Aliases
type CopyFileContentsResult

// Functions
function isProbablyBinary(bytes: Uint8Array) : boolean
function isProbablyBinary(bytes: Uint8Array) : boolean

// Constants
const len
const controlCount
const i
const b

// Functions
function readPrefix(fd: fs.promises.FileHandle, bytes: number) : Promise<Uint8Array>

// Constants
const toRead
const buffer

// Functions
function readFileContentsForClipboard(params: {
  runRoot: string;
  fsPath: string;
  maxBytes?: number;
}) : Promise<CopyFileContentsResult>
function readFileContentsForClipboard(params: {
  runRoot: string;
  fsPath: string;
  maxBytes?: number;
}) : Promise<CopyFileContentsResult>

// Constants
const runRoot
const fsPath
const maxBytes
const lst
const errno
const stat : fs.Stats
const errno
const size
const fd : fs.promises.FileHandle | undefined
const prefix
const buffer
const content
```

### File: `packages/vscode-extension/src/core/debouncedBatcher.ts`

- Size: 978 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions, 1 classes
- 39 lines (0 code)

**Signatures:**

```typescript
// Structs/Classes
class DebouncedBatcher
class DebouncedBatcher

// Constants
const batch
```

### File: `packages/vscode-extension/src/core/journal.ts`

- Size: 2105 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 classes, 1 types
- 75 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type JournalTailResult

// Structs/Classes
class JournalTailer
class JournalTailer

// Constants
const stat : fs.Stats
const errno
const size
const truncated
const bytesToRead
const buffer
const bytesRead
const fd
const text
```

### File: `packages/vscode-extension/src/core/jsonl.ts`

- Size: 3832 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 classes, 2 types
- 134 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type JsonlParseError

// Functions
function createJsonlParseError(message: string, line: number, source: string) : JsonlParseError

// Constants
const err

// Functions
function parseJsonLines(text: string) : unknown[]
function parseJsonLines(text: string) : unknown[]

// Constants
const lines
const results : unknown[]
const idx
const lineNo
const raw
const trimmed
const message

// Type Aliases
type JsonlIncrementalResult

// Structs/Classes
class JsonlIncrementalParser
class JsonlIncrementalParser

// Constants
const records : unknown[]
const errors : JsonlParseError[]
const newlineIndex
const line
const trimmed
const lineNo
const message
const records : unknown[]
const errors : JsonlParseError[]
const trimmed
const lineNo
const message
```

### File: `packages/vscode-extension/src/core/processCatalog.ts`

- Size: 7466 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 17 functions, 4 types
- 255 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type ProcessExportKind
type ProcessParam
type ProcessExport
type ProcessCatalog

// Functions
function determineProcessKind(modulePath: string) : ProcessExportKind
function determineProcessKind(modulePath: string) : ProcessExportKind

// Constants
const normalized

// Functions
function cleanDocComment(raw: string) : string

// Constants
const lines
const cleaned

// Functions
function getLeadingDocComment(sourceText: string, node: ts.Node) : string | undefined

// Constants
const ranges
const last
const raw
const cleaned

// Functions
function stringifyNode(sourceFile: ts.SourceFile, node: ts.Node) : string
function getReturnKeysFromFunctionBody(
  sourceFile: ts.SourceFile,
  body: ts.ConciseBody,
) : string[] | undefined

// Constants
const keys
const addObjectKeys : void
const name
const visit : void
const expr : ts.Expression
const list

// Functions
function getParamsFromSignature(
  sourceFile: ts.SourceFile,
  params: readonly ts.ParameterDeclaration[],
) : ProcessParam[]

// Constants
const name

// Functions
function extractProcessExportsFromSource(
  sourceText: string,
  filePath: string,
  modulePath: string,
) : ProcessExport[]
function extractProcessExportsFromSource(
  sourceText: string,
  filePath: string,
  modulePath: string,
) : ProcessExport[]

// Constants
const sourceFile
const kind
const results : ProcessExport[]
const addExport : void
const doc
const entry : ProcessExport
const isExported
const isExported
const exportName
const init

// Functions
function walkFiles(root: string, relative: string, out: string[]) : void

// Constants
const full
const entries
const childRelative

// Functions
function scanProcessCatalog(processesRootPath: string) : ProcessCatalog
function scanProcessCatalog(processesRootPath: string) : ProcessCatalog

// Constants
const files : string[]
const exports : ProcessExport[]
const fullPath
const sourceText
const modulePath
```

### File: `packages/vscode-extension/src/core/promptBuilder.ts`

- Size: 759 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 types
- 18 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type PromptBuilderInvocation

// Functions
function buildGuidedPrompt(invocation: PromptBuilderInvocation) : string
function buildGuidedPrompt(invocation: PromptBuilderInvocation) : string

// Constants
const header
const argsJson
const argsBlock
const attachments
const request
```

### File: `packages/vscode-extension/src/core/run.ts`

- Size: 479 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 types
- 24 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type RunStatus
type RunTimestamps
type RunPaths
type Run
```

### File: `packages/vscode-extension/src/core/runDetailsSnapshot.ts`

- Size: 12858 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 25 functions, 7 types
- 462 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type RunView
type RunFileItem
type TextFileReadResult
type JournalErrorView
type JournalView
type RunDetailsSnapshot

// Functions
function normalizeForPlatform(p: string) : string

// Constants
const resolved

// Functions
function isFsPathInsideRoot(root: string, candidate: string) : boolean
function isFsPathInsideRoot(root: string, candidate: string) : boolean

// Constants
const normalizedRoot
const normalizedCandidate
const rootWithSep

// Functions
function appendRollingWindow(
  current: readonly T[],
  next: readonly T[],
  limit: number,
) : T[]
function appendRollingWindow(
  current: readonly T[],
  next: readonly T[],
  limit: number,
) : T[]

// Constants
const combined

// Functions
function safeStat(filePath: string) : fs.Stats | undefined
function safeLstat(filePath: string) : fs.Stats | undefined
function safeDirReadable(dirPath: string) : { readable: boolean; error?: string }

// Constants
const errno
const message

// Functions
function listFilesRecursive(params: {
  dir: string;
  rootForRel: string;
  maxFiles: number;
}) : RunFileItem[]
function listFilesRecursive(params: {
  dir: string;
  rootForRel: string;
  maxFiles: number;
}) : RunFileItem[]

// Constants
const results : RunFileItem[]
const stack : string[]
const normalizedRoot
const current
const dirents : fs.Dirent[]
const abs
const relPath
const isDirectory
const stat

// Functions
function listFilesRecursiveFilesOnly(params: {
  dir: string;
  rootForRel: string;
  maxFiles: number;
}) : RunFileItem[]

// Constants
const results : RunFileItem[]
const stack : string[]
const normalizedRoot
const current
const dirents : fs.Dirent[]
const abs
const stat
const relPath

// Functions
function listFilesSortedByMtimeDesc(params: {
  dir: string;
  rootForRel: string;
  maxFiles: number;
}) : RunFileItem[]
function listFilesSortedByMtimeDesc(params: {
  dir: string;
  rootForRel: string;
  maxFiles: number;
}) : RunFileItem[]

// Constants
const items
const aTime
const bTime

// Functions
function readTextFileWithLimit(filePath: string, maxBytes: number) : TextFileReadResult
function readTextFileWithLimit(filePath: string, maxBytes: number) : TextFileReadResult

// Constants
const stat
const size
const fd
const toRead
const buffer
const bytesRead
const content

// Functions
function toJournalView(result: JournalTailResult) : JournalView

// Type Aliases
type KeyFilesMeta

// Functions
function buildRunFileItemIfRegularFile(params: {
  runRoot: string;
  fsPath: string;
}) : RunFileItem | undefined

// Constants
const lst
const stat
const relPath

// Functions
function discoverKeyFiles(params: { run: Run; maxRunFiles: number }) : {
  runFiles: RunFileItem[];
  importantFiles: RunFileItem[];
  keyFilesMeta: KeyFilesMeta;
}

// Constants
const runRoot
const runRootExists
const runRootReadable
const runRootError : string | undefined
const stat
const readable
const errno
const message
const rawRunFiles : RunFileItem[]
const truncated
const runFiles
const importantFsPaths
const importantFiles

// Functions
function readRunDetailsSnapshot(params: {
  run: Run;
  journalTailer: JournalTailer;
  existingJournalEntries: readonly unknown[];
  maxJournalEntries: number;
  maxArtifacts: number;
  maxWorkSummaries: number;
  maxPrompts: number;
}) : { snapshot: RunDetailsSnapshot; nextJournalEntries: unknown[] }
function readRunDetailsSnapshot(params: {
  run: Run;
  journalTailer: JournalTailer;
  existingJournalEntries: readonly unknown[];
  maxJournalEntries: number;
  maxArtifacts: number;
  maxWorkSummaries: number;
  maxPrompts: number;
}) : { snapshot: RunDetailsSnapshot; nextJournalEntries: unknown[] }

// Constants
const state
const tail
const nextJournalEntries
const awaitingInput
const workSummaries
const prompts
const runRoot
const candidates
const merged : RunFileItem[]
const seen
const items
const key
const aTime
const bTime
const artifacts
const mainJsStat
const mainJs
const keyFiles
```

### File: `packages/vscode-extension/src/core/runDiscovery.ts`

- Size: 2595 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions
- 94 lines (0 code)

**Signatures:**

```typescript
// Functions
function isExistingDirectory(dirPath: string) : boolean
function statMtimeMs(filePath: string) : number | undefined
function buildRunPaths(runRoot: string) : RunPaths

// Constants
const codeDir

// Functions
function discoverRuns(runsRootPath: string) : Run[]
function discoverRuns(runsRootPath: string) : Run[]

// Constants
const dirents : fs.Dirent[]
const runs : Run[]
const runDirs
const runRoot
const runDirStat : fs.Stats
const paths
const stateResult
const status
const updatedAtCandidates : number[]
const stateMtime
const journalMtime
const updatedAtMs
const createdAtMs
```

### File: `packages/vscode-extension/src/core/runFileChanges.ts`

- Size: 2166 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 4 types
- 70 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type RunFileArea
type RunFileEventType
type RunFileChange
type RunChangeBatch

// Functions
function classifyRunFileChange(params: {
  runsRootPath: string;
  fsPath: string;
  type: RunFileEventType;
}) : RunFileChange | undefined
function classifyRunFileChange(params: {
  runsRootPath: string;
  fsPath: string;
  type: RunFileEventType;
}) : RunFileChange | undefined

// Constants
const runId
const runRoot
const relativeToRun
const parts
const first

// Functions
function toRunChangeBatch(changes: RunFileChange[]) : RunChangeBatch
function toRunChangeBatch(changes: RunFileChange[]) : RunChangeBatch

// Constants
const areasByRunId
const runIds
const areas
```

### File: `packages/vscode-extension/src/core/runId.ts`

- Size: 1316 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 29 lines (0 code)

**Signatures:**

```typescript
// Functions
function isRunId(dirName: string) : boolean
function isRunId(dirName: string) : boolean

// Constants
const trimmed

// Functions
function extractRunIdFromPath(runsRootPath: string, fsPath: string) : string | undefined
function extractRunIdFromPath(runsRootPath: string, fsPath: string) : string | undefined

// Constants
const relative
const firstSegment
```

### File: `packages/vscode-extension/src/core/runPolling.ts`

- Size: 5116 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 19 functions
- 155 lines (0 code)

**Signatures:**

```typescript
// Functions
function listRunIds(runsRootPath: string) : string[]
function listRunIds(runsRootPath: string) : string[]

// Constants
const dirents

// Functions
function parseRunIdTimestampMs(runId: string) : number | undefined

// Constants
const match
const yyyymmdd
const hhmmss
const year
const month
const day
const hour
const minute
const second
const dt
const ms

// Functions
function bestEffortDirTimeMs(dirPath: string, runId: string) : number | undefined

// Constants
const stat
const byBirth
const byCtime
const byMtime

// Functions
function chooseBestNewRunId(params: {
  runsRootPath: string;
  candidates: string[];
  afterTimeMs?: number;
}) : string

// Constants
const afterMs
const scored
const dirPath
const timeMs
const deltaMs
const nonNegative
const withTime

// Functions
function waitForNewRunId(params: {
  runsRootPath: string;
  baselineIds: Set<string>;
  timeoutMs: number;
  pollIntervalMs?: number;
  signal?: AbortSignal;
  /**
   * Optional timestamp used to disambiguate when multiple new run directories exist.
   * If provided, selects the new run whose (best-effort) directory timestamp is closest after this time.
   */
  afterTimeMs?: number;
  /**
   * Number of consecutive polls the chosen id must remain the best candidate before returning.
   * Default: 2 when `afterTimeMs` is set; otherwise 1.
   */
  stablePolls?: number;
}) : Promise<string | undefined>
function waitForNewRunId(params: {
  runsRootPath: string;
  baselineIds: Set<string>;
  timeoutMs: number;
  pollIntervalMs?: number;
  signal?: AbortSignal;
  /**
   * Optional timestamp used to disambiguate when multiple new run directories exist.
   * If provided, selects the new run whose (best-effort) directory timestamp is closest after this time.
   */
  afterTimeMs?: number;
  /**
   * Number of consecutive polls the chosen id must remain the best candidate before returning.
   * Default: 2 when `afterTimeMs` is set; otherwise 1.
   */
  stablePolls?: number;
}) : Promise<string | undefined>

// Constants
const pollIntervalMs
const stablePolls
const start
const lastPick : string | undefined
const stableCount
const sleep : Promise<void>
const signal
const timer
const onAbort : void
const current
const candidates
const pick
```

### File: `packages/vscode-extension/src/core/runPresentation.ts`

- Size: 788 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 37 lines (0 code)

**Signatures:**

```typescript
// Functions
function formatRunStatus(status: RunStatus) : string
function formatRunStatus(status: RunStatus) : string
function runStatusThemeIconId(status: RunStatus) : string
function runStatusThemeIconId(status: RunStatus) : string
```

### File: `packages/vscode-extension/src/core/runStatus.ts`

- Size: 581 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 25 lines (0 code)

**Signatures:**

```typescript
// Functions
function normalizeRunStatus(raw: unknown) : RunStatus
function normalizeRunStatus(raw: unknown) : RunStatus

// Constants
const value
```

### File: `packages/vscode-extension/src/core/sdkDispatch.ts`

- Size: 5401 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions, 5 types
- 205 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type DispatchNewRunOptions
type DispatchNewRunResult
type ResumeExistingRunOptions
type ResumeExistingRunResult
type SpawnResult

// Functions
function spawnCollect(
  command: string,
  args: string[],
  options: { cwd?: string; env?: NodeJS.ProcessEnv },
) : Promise<SpawnResult>

// Constants
const child
const stdout
const stderr

// Functions
function extractRunIdFromOutput(text: string) : string | undefined

// Constants
const json
const match

// Functions
function dispatchNewRunViaSdk(
  options: DispatchNewRunOptions,
) : Promise<DispatchNewRunResult>
function dispatchNewRunViaSdk(
  options: DispatchNewRunOptions,
) : Promise<DispatchNewRunResult>

// Constants
const sdkCommand
const sdkArgs
const inputsContent
const tempInputsPath
const fs
const baselineIds
const dispatchStartMs
const spawnOptions : { cwd: string; env?: NodeJS.ProcessEnv }
const result
const runId
const foundRunId
const runRootPath
const runRootPath
const fs

// Functions
function resumeExistingRunViaSdk(
  options: ResumeExistingRunOptions,
) : Promise<ResumeExistingRunResult>
function resumeExistingRunViaSdk(
  options: ResumeExistingRunOptions,
) : Promise<ResumeExistingRunResult>

// Constants
const sdkCommand
const sdkArgs
const runRootPath
const spawnOptions : { cwd: string; env?: NodeJS.ProcessEnv }
const result
```

### File: `packages/vscode-extension/src/core/stateJson.ts`

- Size: 2777 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions, 3 types
- 96 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type StateJson
type StateJsonIssue
type ReadStateJsonResult

// Functions
function isNonEmptyString(value: unknown)
function normalizeStateObject(obj: Record<string, unknown>) : StateJson

// Constants
const state : StateJson

// Functions
function parseStateJsonText(text: string) : ReadStateJsonResult
function parseStateJsonText(text: string) : ReadStateJsonResult

// Constants
const issues : StateJsonIssue[]
const trimmed
const parsed : unknown
const message
const state
const status

// Functions
function readStateJsonFile(filePath: string) : ReadStateJsonResult
function readStateJsonFile(filePath: string) : ReadStateJsonResult

// Constants
const raw
const errno
const message
```

### File: `packages/vscode-extension/src/core/stdinControls.ts`

- Size: 308 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions, 1 types
- 14 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type StdinWritableProcess

// Constants
const ESC
const ENTER

// Functions
function sendEsc(process: StdinWritableProcess) : void
function sendEsc(process: StdinWritableProcess) : void
function sendEnter(process: StdinWritableProcess) : void
function sendEnter(process: StdinWritableProcess) : void
```

### File: `packages/vscode-extension/src/core/terminalSanitize.ts`

- Size: 912 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 23 lines (0 code)

**Signatures:**

```typescript
// Constants
const CSI
const OSC
const DCS_PM_APC
const OTHER_CONTROLS

// Functions
function sanitizeTerminalOutput(text: string) : string
function sanitizeTerminalOutput(text: string) : string
```

### File: `packages/vscode-extension/src/core/textLines.ts`

- Size: 975 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 classes, 1 types
- 39 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type TextLineIncrementalResult

// Structs/Classes
class TextLineIncrementalParser
class TextLineIncrementalParser

// Constants
const lines : string[]
const newlineIndex
const line
```

### File: `packages/vscode-extension/src/core/textTailer.ts`

- Size: 2334 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 classes, 1 types
- 82 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type TextTailResult

// Structs/Classes
class TextFileTailer
class TextFileTailer

// Constants
const stat : fs.Stats
const errno
const size
const truncated
const bytesToRead
const buffer
const bytesRead
const fd
const text
```

### File: `packages/vscode-extension/src/core/workSummaryTailSession.ts`

- Size: 3153 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 1 classes, 1 types
- 117 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type WorkSummaryTailEvent

// Functions
function trimToMaxChars(text: string, maxChars: number) : string
function joinLines(lines: readonly string[]) : string
function statSizeOrThrow(filePath: string) : number

// Constants
const stat

// Structs/Classes
class WorkSummaryTailSession
class WorkSummaryTailSession

// Constants
const size : number
const errno
const start
const seeded
const size : number
const errno
const res
const delta
```

### File: `packages/vscode-extension/src/extension/keyFilesModel.ts`

- Size: 14704 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 38 functions, 5 types
- 421 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type KeyFilesItem
type KeyFilesMeta
type KeyFilesSnapshotAugment

// Functions
function safeStat(p: string) : fs.Stats | undefined
function safeAccessReadable(p: string) : boolean
function computeKeyFilesGroupForRelPath(relPath: string) : string
function computeKeyFilesGroupForRelPath(relPath: string) : string

// Constants
const normalized
const p

// Functions
function toKeyFilesItemFromRunFile(params: { item: RunFileItem; runRoot: string }) : KeyFilesItem

// Constants
const relPath
const displayName

// Functions
function buildImportantFile(params: {
  runRoot: string;
  fsPath: string;
  displayName: string;
  group: string;
}) : KeyFilesItem | undefined

// Constants
const stat
const relPath

// Functions
function buildKeyFilesSnapshotAugment(params: {
  run: Run;
  maxRunFiles?: number;
}) : KeyFilesSnapshotAugment
function buildKeyFilesSnapshotAugment(params: {
  run: Run;
  maxRunFiles?: number;
}) : KeyFilesSnapshotAugment

// Constants
const maxRunFiles
const runRoot
const runRootExists
const runRootReadable
const runRootError : string | undefined
const stat
const message
const rawRunFiles : RunFileItem[]
const truncated
const runFiles
const importantCandidates : Array<{ fsPath: string; displayName: string; group: string }>
const importantFiles

// Functions
function groupOrderIndex(group: string) : number
function groupOrderIndex(group: string) : number

// Constants
const order
const idx

// Functions
function matchesKeyFilesFilter(
  item: { relPath?: unknown; displayName?: unknown } | undefined,
  filterLower: string,
) : boolean
function matchesKeyFilesFilter(
  item: { relPath?: unknown; displayName?: unknown } | undefined,
  filterLower: string,
) : boolean

// Constants
const rel
const name

// Functions
function normalizePinnedIdsByRunId(input: unknown) : Record<string, string[]>
function normalizePinnedIdsByRunId(input: unknown) : Record<string, string[]>

// Constants
const out : Record<string, string[]>
const cleaned : string[]
const seen
const trimmed

// Functions
function getPinnedIdsForRun(pinnedIdsByRunId: unknown, runId: string) : string[]
function getPinnedIdsForRun(pinnedIdsByRunId: unknown, runId: string) : string[]

// Constants
const normalized

// Functions
function setPinnedIdsForRun(
  pinnedIdsByRunId: unknown,
  runId: string,
  ids: readonly string[],
) : Record<string, string[]>
function setPinnedIdsForRun(
  pinnedIdsByRunId: unknown,
  runId: string,
  ids: readonly string[],
) : Record<string, string[]>

// Constants
const normalized
const next : Record<string, string[]>
const cleaned

// Functions
function togglePinnedId(ids: readonly string[], fileId: string) : string[]
function togglePinnedId(ids: readonly string[], fileId: string) : string[]

// Constants
const current

// Type Aliases
type GroupedKeyFiles

// Functions
function groupKeyFiles(items: readonly KeyFilesItem[]) : GroupedKeyFiles
function groupKeyFiles(items: readonly KeyFilesItem[]) : GroupedKeyFiles

// Constants
const groups : Record<string, KeyFilesItem[]>
const group
const groupKeys
const ai
const bi
const sorted
const ar
const br
const byRel

// Type Aliases
type KeyFilesModel

// Functions
function isRecord(value: unknown)
function getString(value: unknown) : string | undefined
function getNumberOrNull(value: unknown) : number | null
function getBoolean(value: unknown) : boolean | undefined
function computeKeyFilesModel(params: {
  snapshot: unknown;
  filterValue: string;
  pinnedIdsByRunId: unknown;
}) : KeyFilesModel
function computeKeyFilesModel(params: {
  snapshot: unknown;
  filterValue: string;
  pinnedIdsByRunId: unknown;
}) : KeyFilesModel

// Constants
const snapshot
const snapshotRecord
const runRecord
const runId
const runPathsRecord
const runRoot
const keyFilesMetaRecord
const runRootError
const keyFilesMeta : KeyFilesMeta | undefined
const runRootExists
const runRootReadable
const runFilesRaw : unknown[]
const importantFilesRaw : unknown[]
const normalizeItem : KeyFilesItem | undefined
const relPath
const fsPath
const id
const isDirectory
const group
const displayName
const exists
const runFiles
const byId
const pinnedIds
const pinnedItems : KeyFilesItem[]
const keptPinnedIds : string[]
const item
const pinnedSet
const importantItems
const importantSet
const remainingItems
const filterLower
const filteredPinned
const filteredImportant
const filteredRemaining
const pillSuffix
const pillText
const hasAny
const anyFiltered
const emptyMessage : string | undefined
```

### File: `packages/vscode-extension/src/extension/promptBuilderBridge.ts`

- Size: 7947 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions, 1 classes, 3 types
- 222 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type PromptBuilderPersistedState
type PromptBuilderMementoLike
type PromptBuilderWebviewLike

// Constants
const STATE_KEY

// Functions
function parseDroppedText(raw: string) : string[]
function parseDroppedText(raw: string) : string[]
function looksLikeUri(value: string) : boolean
function tryFsPathFromDropItem(item: string) : { fsPath: string; link?: string } | undefined

// Constants
const trimmed
const url
const decodedPath

// Functions
function normalizeDropItemsToAttachmentRefs(
  items: readonly string[],
  workspaceRoot: string,
) : string[]
function normalizeDropItemsToAttachmentRefs(
  items: readonly string[],
  workspaceRoot: string,
) : string[]

// Constants
const normalizedRoot
const out : string[]
const resolved
const candidateFsPath
const rel
const relPosix

// Functions
function sanitizePersistedState(raw: unknown) : PromptBuilderPersistedState | undefined
function sanitizePersistedState(raw: unknown) : PromptBuilderPersistedState | undefined

// Constants
const obj
const selectedId
const request
const attachments
const paramValuesRaw
const paramValues : Record<string, string>

// Functions
function mergeUniqueStrings(
  existing: readonly string[],
  incoming: readonly string[],
) : string[]
function mergeUniqueStrings(
  existing: readonly string[],
  incoming: readonly string[],
) : string[]

// Constants
const out

// Structs/Classes
class PromptBuilderBridge
class PromptBuilderBridge

// Constants
const saved
const sanitized
const typed
const raw
const text
const responseType
const raw
const items
const normalized
const sanitized
```

### File: `packages/vscode-extension/src/extension/promptBuilderView.ts`

- Size: 28289 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions, 2 types
- 880 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type PromptBuilderWireExport
type PromptBuilderWireCatalog

// Functions
function toWireCatalog(catalog: ProcessCatalog) : PromptBuilderWireCatalog

// Constants
const base : PromptBuilderWireExport

// Functions
function getNonce() : string

// Constants
const chars
const result
const i

// Functions
function htmlForWebview(webview: vscode.Webview, title: string) : string

// Constants
const nonce
const csp

// Functions
function registerPromptBuilderCommand(context: vscode.ExtensionContext) : vscode.Disposable
function registerPromptBuilderCommand(context: vscode.ExtensionContext) : vscode.Disposable

// Constants
const promptBuilderPanel : vscode.WebviewPanel | undefined

// Functions
function openPromptBuilder(context: vscode.ExtensionContext) : Promise<void>
function openPromptBuilder(context: vscode.ExtensionContext) : Promise<void>

// Constants
const workspaceRoot
const processesRootPath
const bridge
const typed
const exists
const catalog
const processId
const args
const request
const attachments
const text
const text
const editor
const text

// Functions
function fsExists(p: string) : boolean
```

### File: `packages/vscode-extension/src/extension/runDetailsView.ts`

- Size: 89537 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions, 1 classes, 3 types
- 2571 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type WebviewInboundMessage
type WebviewOutboundMessage
type RunInteractionController

// Functions
function nonce(len = 16) : string

// Constants
const alphabet
const out
const i

// Functions
function renderWebviewHtml(webview: vscode.Webview) : string

// Constants
const cspSource
const scriptNonce
const webviewHelpersJs

// Structs/Classes
class RunDetailsPanel

// Constants
const readRunDetailsSnapshot
const processAwaiting
const pid
const label
const tail
const update
const message
const text
const trimmed
const ok
const ok
const ok
const value
const from
const spec
const isRelative
const isRunAbsolute
const baseDir
const base
const candidates
const resolved : string | undefined
const st
const value
const message
const value
const sourceUri
const stat : vscode.FileStat
const basename
const defaultUri
const destUri
const MAX_FALLBACK_BYTES
const data
const message
const res
const message
const res
const value
const value
const res

// Functions
function createRunDetailsViewManager(
  context: vscode.ExtensionContext,
  params?: { interaction?: RunInteractionController },
) : {
  open: (run: Run) => Promise<void>;
  onRunChangeBatch: (batch: RunChangeBatch) => void;
}
function createRunDetailsViewManager(
  context: vscode.ExtensionContext,
  params?: { interaction?: RunInteractionController },
) : {
  open: (run: Run) => Promise<void>;
  onRunChangeBatch: (batch: RunChangeBatch) => void;
}

// Constants
const panelsByRunId
const interaction
const panel
const panel
const open : Promise<void>
const existing
const panel
const onRunChangeBatch : void
const panel
```

### File: `packages/vscode-extension/src/extension/runFileWatchers.ts`

- Size: 3462 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 18 functions
- 99 lines (0 code)

**Signatures:**

```typescript
// Functions
function toPosixGlobPath(p: string) : string
function createGlobPatterns(params: {
  runsRootPath: string;
  workspaceFolder: vscode.WorkspaceFolder | undefined;
}) : { base: vscode.Uri; prefix: string }

// Constants
const workspaceRoot
const relative
const isInsideWorkspace
const prefix

// Functions
function createRunFileWatchers(params: {
  runsRootPath: string;
  workspaceFolder: vscode.WorkspaceFolder | undefined;
  debounceMs: number;
  onBatch: (batch: RunChangeBatch) => void;
}) : vscode.Disposable
function createRunFileWatchers(params: {
  runsRootPath: string;
  workspaceFolder: vscode.WorkspaceFolder | undefined;
  debounceMs: number;
  onBatch: (batch: RunChangeBatch) => void;
}) : vscode.Disposable

// Constants
const globs
const batcher
const handle : void
const classified
const stateWatcher
const journalWatcher
const artifactsWatcher
const workSummariesWatcher
const disposables : vscode.Disposable[]
```

### File: `packages/vscode-extension/src/extension/runLogsView.ts`

- Size: 20074 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions, 1 classes, 4 types
- 666 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type LogSourceId
type WebviewInboundMessage
type WebviewOutboundMessage

// Functions
function nonce(len = 16) : string

// Constants
const alphabet
const out
const i

// Functions
function renderWebviewHtml(webview: vscode.Webview) : string

// Constants
const cspSource
const scriptNonce

// Functions
function trimToMaxChars(text: string, maxChars: number) : string
function seedTextTailerFromEnd(params: {
  tailer: TextFileTailer;
  filePath: string;
  maxBytes: number;
}) : string

// Constants
const stat : fs.Stats | undefined
const start
const seeded

// Functions
function findLatestRegularFile(candidates: string[]) : string | undefined

// Type Aliases
type Candidate

// Constants
const found : Candidate[]
const dirents : fs.Dirent[]
const fsPath
const stat

// Structs/Classes
class RunLogsPanel

// Constants
const sources : Array<{ id: LogSourceId; label: string }>
const stat
const stat
const areas
const tail
const latest
const tail

// Functions
function createRunLogsViewManager(
  context: vscode.ExtensionContext,
  deps: { output: vscode.OutputChannel },
) : { open: (run: Run) => void; onRunChangeBatch: (batch: RunChangeBatch) => void }
function createRunLogsViewManager(
  context: vscode.ExtensionContext,
  deps: { output: vscode.OutputChannel },
) : { open: (run: Run) => void; onRunChangeBatch: (batch: RunChangeBatch) => void }

// Constants
const panelsByRunId
const open : void
const existing
const panel
const onRunChangeBatch : void
```

### File: `packages/vscode-extension/src/extension/runTreeView.ts`

- Size: 10506 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 23 functions, 3 classes, 1 types
- 323 lines (0 code)

**Signatures:**

```typescript
// Type Aliases
type RunTreeItemOrRun

// Functions
function isRunTreeItem(value: unknown)
function toRun(value: unknown) : Run | undefined

// Structs/Classes
class RunTreeItem
class MessageTreeItem
class RunsTreeDataProvider

// Functions
function pickRun(runs: Run[]) : Promise<Run | undefined>

// Constants
const pick

// Functions
function openStateJson(run: Run) : Promise<void>

// Constants
const uri

// Functions
function getRunsArchiveRootFromRun(run: Run) : string

// Constants
const runsRoot

// Functions
function archiveRun(run: Run) : Promise<void>

// Constants
const choice
const archiveRoot
const dest
const message
const message

// Functions
function markRunComplete(run: Run) : Promise<void>

// Constants
const choice
const raw
const message
const parsed : unknown
const message
const message

// Functions
function registerRunsTreeView(context: vscode.ExtensionContext) : {
  setRunsRootPath: (runsRootPath: string | undefined) => void;
  refresh: () => void;
  getRunsSnapshot: () => Run[];
  setOpenRunDetailsHandler: (handler: ((run: Run) => Promise<void>) | undefined) => void;
  setOpenRunLogsHandler: (handler: ((run: Run) => Promise<void>) | undefined) => void;
}
function registerRunsTreeView(context: vscode.ExtensionContext) : {
  setRunsRootPath: (runsRootPath: string | undefined) => void;
  refresh: () => void;
  getRunsSnapshot: () => Run[];
  setOpenRunDetailsHandler: (handler: ((run: Run) => Promise<void>) | undefined) => void;
  setOpenRunLogsHandler: (handler: ((run: Run) => Promise<void>) | undefined) => void;
}

// Constants
const provider
const refreshDisposable
const defaultOpenRunDetails : Promise<void>
const openRunDetailsHandler : (run: Run) => Promise<void>
const defaultOpenRunLogs : Promise<void>
const uri
const openRunLogsHandler : (run: Run) => Promise<void>
const openRunDetailsDisposable
const fromArg
const run
const openRunLogsDisposable
const fromArg
const run
const revealRunFolderDisposable
const fromArg
const run
const message
const archiveRunDisposable
const fromArg
const run
const markCompleteDisposable
const fromArg
const run
```

### File: `packages/vscode-extension/src/extension.ts`

- Size: 20761 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 37 functions, 1 types
- 553 lines (0 code)

**Signatures:**

```typescript
// Constants
const OUTPUT_CHANNEL_NAME

// Type Aliases
type BabysitterApi

// Functions
function readRawSettings() : BabysitterRawSettings

// Constants
const cfg

// Functions
function formatIssues(issues: ConfigIssue[]) : string[]

// Constants
const detail

// Functions
function toRunFromCommandArg(value: unknown) : Run | undefined

// Constants
const candidate

// Functions
function sanitizeForOutputChannel(text: string) : string
function toUriFromExplorerArg(value: unknown) : vscode.Uri | undefined

// Constants
const uri

// Functions
function activate(context: vscode.ExtensionContext) : BabysitterApi
function activate(context: vscode.ExtensionContext) : BabysitterApi

// Constants
const output
const interactionController : RunInteractionController
const runDetailsView
const runLogsView
const runsTreeView
const status
const lastNotifiedFingerprint
const runWatchersDisposable : vscode.Disposable | undefined
const openPromptBuilderDisposable
const mod
const message
const refreshConfig : Promise<void>
const workspaceRoot
const workspaceFolder
const settings
const resolveOptions : Parameters<typeof resolveBabysitterConfig>[0]
const result
const fingerprint
const action
const choice
const runsRootPath
const summary
const areas
const areaList
const disposable
const dispatchRunDisposable
const workspaceFolder
const workspaceRoot
const settings
const resolveOptions : Parameters<typeof resolveBabysitterConfig>[0]
const result
const promptFromArg
const prompt
const dispatched
const message
const dispatchRunFromTaskFileDisposable
const uri
const promptText : string
const message
const message
const resumeRunDisposable
const workspaceFolder
const workspaceRoot
const settings
const resolveOptions : Parameters<typeof resolveBabysitterConfig>[0]
const result
const fromRun
const runIdFromArg
const promptFromArg
const runId
const runs
const picked
const prompt
const resumed
const message
const showConfigErrorsDisposable
const workspaceRoot
const settings
const resolveOptions : Parameters<typeof resolveBabysitterConfig>[0]
const result
const resolvedSdkBinary
const resolvedRunsRoot
const action
const choice

// Functions
function deactivate() : void
function deactivate() : void
```

### File: `packages/vscode-extension/src/unit/awaitingInput.test.ts`

- Size: 2209 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions
- 57 lines (0 code)

**Signatures:**

```typescript
// Functions
function fixturePath(rel: string) : string

// Constants
const state
const detected
const raw
const entries
const detected
const raw
const output
const detected
const output
```

### File: `packages/vscode-extension/src/unit/copyFileContents.test.ts`

- Size: 1924 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 53 lines (0 code)

**Signatures:**

```typescript
// Functions
function mkTmpDir() : string

// Constants
const runRoot
const filePath
const res
const runRoot
const filePath
const res
const runRoot
const filePath
const res
const runRoot
const otherRoot
const filePath
const res
```

### File: `packages/vscode-extension/src/unit/debouncedBatcher.test.ts`

- Size: 1746 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 65 lines (0 code)

**Signatures:**

```typescript
// Functions
function sleep(ms: number) : Promise<void>

// Constants
const batches : number[][]
const batcher
const batches : number[][]
const batcher
const batches : number[][]
const batcher
const batches : number[][]
const batcher
```

### File: `packages/vscode-extension/src/unit/journal.test.ts`

- Size: 3810 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions
- 103 lines (0 code)

**Signatures:**

```typescript
// Functions
function fixturePath(...parts: string[]) : string
function makeTempDir(prefix: string) : string

// Constants
const sample
const tempDir
const journalPath
const tailer
const first
const second
const prefix
const suffix
const tempDir
const journalPath
const marker
const markerIndex
const partialPrefix
const tailer
const first
const second
const sample
const tempDir
const journalPath
const tailer
const result
const tempDir
const journalPath
const tailer
const first
const second
```

### File: `packages/vscode-extension/src/unit/jsonl.test.ts`

- Size: 1210 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 34 lines (0 code)

**Signatures:**

```typescript
// Constants
const input
const parsed
const input
const parsed
const input
const anyErr
```

### File: `packages/vscode-extension/src/unit/keyFilesModel.test.ts`

- Size: 4110 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions
- 140 lines (0 code)

**Signatures:**

```typescript
// Constants
const base
const grouped
const code
const pinnedIdsByRunId
const snapshot : unknown
const model
const updated
const missing : unknown
const missingModel
const unreadable : unknown
const unreadableModel
```

### File: `packages/vscode-extension/src/unit/manifest.test.ts`

- Size: 4185 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 107 lines (0 code)

**Signatures:**

```typescript
// Functions
function isRecord(value: unknown)
function assertIsRecord(
  value: unknown,
  message = 'expected an object',
)
function readPackageJson() : unknown

// Constants
const root
const raw

// Functions
function readVscodeIgnore() : string

// Constants
const root
const pkg
const activationEvents
const contributes
const commands
const keybindings
const required
const explorerValue
const explorer
const pkg
const ignore
```

### File: `packages/vscode-extension/src/unit/processCatalog.test.ts`

- Size: 2916 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 76 lines (0 code)

**Signatures:**

```typescript
// Constants
const src
const exports
const first
const src
const exports
const first
```

### File: `packages/vscode-extension/src/unit/promptBuilder.test.ts`

- Size: 1058 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 25 lines (0 code)

**Signatures:**

```typescript
// Constants
const prompt
```

### File: `packages/vscode-extension/src/unit/promptBuilderBridgePreview.test.ts`

- Size: 1792 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 48 lines (0 code)

**Signatures:**

```typescript
// Functions
function makeWebview() : { webview: PromptBuilderWebviewLike; messages: unknown[] }

// Constants
const messages : unknown[]
const makeWebview
const bridge
const handled
const makeWebview
const bridge
const handled
const makeWebview
const bridge
const handled
```

### File: `packages/vscode-extension/src/unit/promptBuilderViewPreviewButton.test.ts`

- Size: 3726 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions
- 94 lines (0 code)

**Signatures:**

```typescript
// Functions
function readWorkspaceFile(relPath: string) : string

// Constants
const root
const source
const source
const source
const source
const source
```

### File: `packages/vscode-extension/src/unit/runDetailsSnapshot.test.ts`

- Size: 8535 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 14 functions
- 209 lines (0 code)

**Signatures:**

```typescript
// Functions
function makeTempDir(prefix: string) : string

// Constants
const tempDir
const root
const inside
const outside
const tempDir
const dir
const items
const rels
const tempDir
const dir
const a
const b
const items
const tempDir
const runRoot
const run : Run
const readRunDetailsSnapshot
const rels
const importantRels
const tempDir
const runRoot
const run : Run
const readRunDetailsSnapshot
const importantRels
```

### File: `packages/vscode-extension/src/unit/runDetailsViewWorkSummaryPreviewContract.test.ts`

- Size: 4564 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions
- 107 lines (0 code)

**Signatures:**

```typescript
// Functions
function readWorkspaceFile(relPath: string) : string

// Constants
const root
const source
const source
const source
const source
const source
```

### File: `packages/vscode-extension/src/unit/runDiscovery.test.ts`

- Size: 4255 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions
- 116 lines (0 code)

**Signatures:**

```typescript
// Functions
function makeTempDir(prefix: string) : string
function writeJson(filePath: string, value: unknown) : void

// Constants
const tempDir
const missingRunsRoot
const tempDir
const runsRoot
const runA
const runPlus
const runWeird
const runs
const byId
const tempDir
const runsRoot
const runMissingState
const runInvalidState
const runs
const tempDir
const runsRoot
const older
const newer
const runs
```

### File: `packages/vscode-extension/src/unit/runFileChanges.test.ts`

- Size: 4830 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions
- 146 lines (0 code)

**Signatures:**

```typescript
// Functions
function makeTempDir(prefix: string) : string

// Constants
const tempDir
const runsRoot
const runId
const statePath
const change
const tempDir
const runsRoot
const runId
const journalPath
const change
const tempDir
const runsRoot
const runId
const artifactPath
const change
const tempDir
const runsRoot
const runId
const workPath
const change
const tempDir
const runsRoot
const runId
const outside
const rootFile
const unrelated
const changes : RunFileChange[]
const batch
```

### File: `packages/vscode-extension/src/unit/runPolling.test.ts`

- Size: 1631 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 46 lines (0 code)

**Signatures:**

```typescript
// Functions
function makeTempDir(prefix: string) : string

// Constants
const tempDir
const runsRootPath
const baselineIds
const afterTimeMs
const runEarly
const runLate
const earlyPath
const latePath
const found
```

### File: `packages/vscode-extension/src/unit/runPresentation.test.ts`

- Size: 1058 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 23 lines (0 code)

### File: `packages/vscode-extension/src/unit/sourceSanity.test.ts`

- Size: 906 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 25 lines (0 code)

**Signatures:**

```typescript
// Functions
function assertAsciiOnly(fileRelPath: string) : void

// Constants
const root
const abs
const text
const i
const code
const snippet
```

### File: `packages/vscode-extension/src/unit/stateJson.test.ts`

- Size: 1967 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 51 lines (0 code)

**Signatures:**

```typescript
// Functions
function fixturePath(...parts: string[]) : string
function makeTempDir(prefix: string) : string

// Constants
const text
const result
const text
const result
const text
const result
const tempDir
const missing
const result
```

### File: `packages/vscode-extension/src/unit/terminalSanitize.test.ts`

- Size: 402 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 11 lines (0 code)

**Signatures:**

```typescript
// Constants
const input
const output
```

### File: `packages/vscode-extension/src/unit/textTailer.test.ts`

- Size: 2899 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 81 lines (0 code)

**Signatures:**

```typescript
// Functions
function fixturePath(...parts: string[]) : string
function makeTempDir(prefix: string) : string

// Constants
const sample
const tempDir
const logPath
const tailer
const first
const second
const prefix
const suffix
const tempDir
const logPath
const marker
const markerIndex
const partialPrefix
const tailer
const first
const second
const tempDir
const logPath
const tailer
const first
const second
```

### File: `packages/vscode-extension/src/unit/workSummaryTailSession.test.ts`

- Size: 3239 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 83 lines (0 code)

**Signatures:**

```typescript
// Functions
function makeTempDir(prefix: string) : string

// Constants
const tempDir
const filePath
const session
const start
const update
const tempDir
const filePath
const session
const start
const session
const missingPath
const start
const tempDir
const filePath
const session
const first
const update
```

### File: `plugins/babysitter/skills/babysit/process/gsd/audit-milestone.js`

- Size: 2708 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 88 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const auditResult
const auditMilestoneTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/discuss-phase.js`

- Size: 10687 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 321 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const decisionPointsResult
const preferencesResult
const contextResult
const identifyDecisionPointsTask
const capturePreferencesTask
const generateContextTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/execute-phase.js`

- Size: 10065 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 319 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const results
const commits
const waves
const waveIndex
const wave
const waveResults
const taskResult
const verifyResult
const commitResult
const allTasksSucceeded
const phaseVerification

// Functions
function organizeIntoWaves((plans))

// Constants
const executeTaskTask
const verifyTaskTask
const createCommitTask
const verifyPhaseTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/iterative-convergence.js`

- Size: 3876 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 128 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const iteration
const quality
const converged
const results
const impl
const tests
const qualityResult
const implementTask
const testTask
const qualityScoringTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/map-codebase.js`

- Size: 6208 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 196 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const structure
const integration
const analyzeStructureTask
const analyzePatternsTask
const analyzeDependenciesTask
const analyzeTechStackTask
const createIntegrationPlanTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/new-project.js`

- Size: 20424 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 593 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const visionResult
const stackResearch
const research
const requirementsResult
const roadmapResult
const visionCaptureTask
const stackResearchTask
const featuresResearchTask
const architectureResearchTask
const pitfallsResearchTask
const requirementsScopingTask
const roadmapCreationTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/plan-phase.js`

- Size: 11647 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 360 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const researchResult
const iteration
const verified
const plans
const feedback
const planningResult
const verificationResult
const targetedResearchTask
const generatePlansTask
const verifyPlansTask
const logFeedbackTask
```

### File: `plugins/babysitter/skills/babysit/process/gsd/verify-work.js`

- Size: 12222 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 391 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const checklistResult
const uatResult
const hasIssues
const diagnosisResult
const fixPlansResult
const fixResults
const fixResult
const prepareChecklistTask
const diagnoseIssuesTask
const createFixPlansTask
const executeFixTask
const commitFixTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/adversarial-spec-debates.js`

- Size: 14474 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 437 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const round
const specApproved
const debateHistory
const currentSpec
const initialSpec
const challenges
const i
const role
const challenge
const refinedSpec
const judgement
const finalJudgement
const agentProposeSpecTask
const agentChallengeTask
const agentRefineSpecTask
const agentJudgeTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/agile.js`

- Size: 20171 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 642 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const currentBacklog
const sprints
const allDeliverables
const velocityHistory
const sprintNumber
const sprintData
const planning
const developmentResults
const devResult
const testResults
const integrationResult
const review
const retrospective
const release
const totalCommittedPoints
const totalCompletedPoints
const overallVelocity

// Functions
function calculateVelocityTrend((history))

// Constants
const recentVelocities
const avgRecent
const earlyVelocities
const avgEarly
const agentSprintPlanningTask
const agentDevelopStoryTask
const agentTestSprintTask
const agentIntegrateTask
const agentSprintReviewTask
const agentRetrospectiveTask
const agentReleaseTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/atdd-tdd/atdd-tdd.js`

- Size: 42822 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1197 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const criteriaResult
const acceptanceTestsResult
const initialAcceptanceRunResult
const tddCycles
const allUnitTests
const implementationFiles
const acceptanceTestsPassing
const iteration
const cycleData
const unitTestResult
const unitTestRunResult
const implementationResult
const existingFileIndex
const postImplUnitTestRun
const fixResult
const refactorResult
const fileIndex
const postRefactorTestRun
const fileIndex
const currentAcceptanceRun
const passingCount
const totalCount
const integrationTestsResult
const integrationRunResult
const coverageResult
const defineAcceptanceCriteriaTask
const createAcceptanceTestsTask
const runAcceptanceTestsTask
const createUnitTestTask
const implementCodeTask
const refactorCodeTask
const runUnitTestsTask
const createIntegrationTestsTask
const runIntegrationTestsTask
const calculateCoverageTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/base44.js`

- Size: 18255 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 563 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const conversationHistory
const currentApp
const userSatisfied
const initialGeneration
const round
const userFeedback
const refinedApp
const deployment
const validation
const agentGenerateAppFromPromptTask
const agentGatherFeedbackTask
const agentRefineAppTask
const agentGenerateDeploymentTask
const agentValidateAppTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/bdd-specification-by-example/bdd-process.js`

- Size: 42843 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions
- 1249 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const discoveryResult
const clarifiedDiscovery
const clarificationResult
const gherkinResult
const scenarioAnalysisResult
const stepDefinitionResult
const reuseAnalysisResult
const automationResult
const stepGroups
const implementedSteps
const groupIdx
const group
const groupResults
const testDataResult
const testExecutionResult
const documentationResult
const traceabilityResult
const allScenariosPassing

// Functions
function getStepFileExtension((framework))

// Constants
const extensions

// Functions
function getLanguage((framework))

// Constants
const languages

// Functions
function groupStepsByComplexity((steps))

// Constants
const groups
const complexity

// Functions
function estimateStepComplexity((step))

// Constants
const text
const discoveryWorkshopTask
const clarifyQuestionsTask
const gherkinFormulationTask
const analyzeScenarioQualityTask
const stepDefinitionTask
const analyzeStepReuseTask
const implementStepTask
const generateTestDataTask
const executeTestsTask
const generateLivingDocumentationTask
const generateTraceabilityTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/bottom-up.js`

- Size: 22545 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 726 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const componentLibrary
const compositionTree
const primitiveAnalysis
const level0Components
const component
const tests
const tests
const currentLevel
const previousLevelComponents
const systemComplete
const compositionPlan
const currentLevelComponents
const composedComponent
const tests
const tests
const completionCheck
const finalIntegration
const validation
const agentIdentifyPrimitivesTask
const agentWriteTestsTask
const agentImplementComponentBottomUpTask
const agentTestComponentTask
const agentPlanCompositionTask
const agentComposeComponentTask
const agentCheckSystemCompletionTask
const agentIntegrateSystemTask
const agentValidateBottomUpSystemTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/build-realtime-remediation.js`

- Size: 14519 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 505 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const attempt
const buildSucceeded
const testsSucceeded
const remediationHistory
const buildResult
const analysis
const remediation
const testAttempt
const testResult
const analysis
const remediation
const overallSuccess
const executeBuildTask
const args
const require
const require
const execAsync
const executeTestsTask
const args
const require
const require
const execAsync

// Functions
function extractErrors((output))

// Constants
const errorPatterns
const errors
const lines

// Functions
function extractFailedTests((output))

// Constants
const failedTests
const failPattern
const lines
const match
const agentAnalyzeFailureTask
const agentRemediateTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/cleanroom/cleanroom.js`

- Size: 58126 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1494 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const specificationResult
const incrementPlanResult
const developmentResults
const cumulativeCode
const i
const increment
const incrementNumber
const designResult
const fixedDesignResult
const implementationResult
const inspectionResult
const fixedImplementation
const usageModelResult
const testGenerationResult
const testExecutionResult
const reliabilityAnalysisResult
const certificationResult
const createFormalSpecificationTask
const planIncrementsTask
const designWithVerificationTask
const fixDesignTask
const implementIncrementTask
const codeInspectionTask
const fixImplementationTask
const createUsageModelTask
const generateStatisticalTestsTask
const executeStatisticalTestsTask
const analyzeReliabilityTask
const generateCertificationTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/consensus-and-voting-mechanisms.js`

- Size: 11111 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 340 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const round
const consensusReached
const rounds
const proposals
const i
const proposal
const votes
const voterId
const vote
const tallyResult
const finalRound
const success
const agentProposeTask
const agentVoteTask
const tallyVotesTask
const args
const winningProposal
const consensusPercentage
const consensusReached
const rankings
const scores
const maxScore
const winnerId
const voteCounts
const maxVotes
const winnerId
const id
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/devin.js`

- Size: 12373 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 411 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const plan
const code
const debugIteration
const allTestsPass
const quality
const debugResults
const testResult
const failedTests
const debugAnalysis
const fixResult
const qualityResult
const qualityMet
const deployment
const agentPlanningTask
const codingTask
const testTask
const agentDebugTask
const applyFixesTask
const agentQualityScoringTask
const deployTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/domain-driven-design/domain-driven-design.js`

- Size: 56760 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1553 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const results
const subdomainResult
const boundedContextResult
const contextMapResult
const initialLanguageResult
const loadedModel
const boundedContexts
const tacticalResults
const entityVOResult
const aggregateResult
const repositoryResult
const domainServiceResult
const domainEventResult
const eventHandlerResult
const refinedLanguageResult
const languageValidation
const boundedContexts
const implementationPlans
const contextTactical
const finalValidation
const identifySubdomainsTask
const defineBoundedContextsTask
const createContextMapTask
const buildUbiquitousLanguageTask
const identifyEntitiesValueObjectsTask
const defineAggregatesTask
const designRepositoriesTask
const identifyDomainServicesTask
const identifyDomainEventsTask
const modelEventHandlersTask
const validateLanguageConsistencyTask
const generateImplementationTask
const validateDomainModelTask
const loadDomainModelTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/double-diamond/double-diamond.js`

- Size: 40742 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1058 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const results
const discoveryResult
const loadedDiscovery
const definitionResult
const loadedData
const developmentResult
const loadedData
const deliveryResult
const iterationDecision
const finalValidation
const discoverTask
const defineTask
const developTask
const deliverTask
const iterationDecisionTask
const validateDoubleDiamondTask
const loadArtifactsTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/event-storming/event-storming.js`

- Size: 30620 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 847 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const bigPictureResult
const processModelingResult
const softwareDesignResult
const contextMappingResult
const visualizationResult
const bigPictureStormingTask
const processModelingTask
const softwareDesignTask
const contextMappingTask
const visualizationTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/evolutionary.js`

- Size: 16945 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions
- 538 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const generation
const targetReached
const evolutionHistory
const initialPopulation
const currentPopulation
const fitnessResults
const parents
const offspring
const mutatedOffspring
const combinedPopulation
const bestSolution

// Functions
function evaluatePopulationFitness((ctx, task, population, generation))

// Constants
const fitnessEvaluations
const i
const individual
const fitness
const ranked

// Functions
function calculateDiversity((population))

// Constants
const uniqueApproaches

// Functions
function calculateImprovementRate((history))

// Constants
const firstGen
const lastGen

// Functions
function findConvergenceGeneration((history, threshold = 1.0))

// Constants
const i
const recentImprovement
const agentGeneratePopulationTask
const agentEvaluateFitnessTask
const agentSelectParentsTask
const args
const parents
const tournamentSize
const i
const tournament
const j
const winner
const totalFitness
const i
const spin
const agentCrossoverTask
const agentMutateTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/example-mapping/example-mapping.js`

- Size: 26346 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 789 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const sessionStartTime
const storyAnalysis
const ruleExtraction
const rules
const exampleResults
const result
const allExamples
const coverageCheck
const ruleExamples
const uncoveredRules
const questionIdentification
const questions
const gherkinGeneration
const gherkinScenarios
const sessionEndTime
const sessionDurationMinutes
const readinessAssessment
const storyAnalysisTask
const ruleExtractionTask
const exampleGenerationTask
const questionIdentificationTask
const gherkinGenerationTask
const readinessAssessmentTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/extreme-programming/xp-process.js`

- Size: 49757 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1390 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const releasePlanResult
const iterations
const completedStories
const currentVelocity
const iterationNum
const availableStories
const iterationPlanResult
const dailyResults
const daysInIteration
const day
const standupResult
const practiceResults
const pairResult
const tddResult
const ciResult
const refactoringResult
const iterationCompleteResult
const retrospectiveResult
const releaseCompleteResult
const practiceMetricsResult
const releasePlanningTask
const iterationPlanningTask
const standUpMeetingTask
const pairProgrammingTask
const tddPracticeTask
const continuousIntegrationTask
const refactoringTask
const iterationCompletionTask
const retrospectiveTask
const releaseCompletionTask
const practiceMetricsTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/feature-driven-development/feature-driven-development.js`

- Size: 31535 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 851 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const domainModelResult
const featuresListResult
const planByFeatureResult
const iterationResults
const iterIdx
const iteration
const iterationData
const designBuildResults
const featureResults
const feature
const designResult
const buildResult
const parkingLotResult
const totalFeaturesInIteration
const completedFeaturesInIteration
const allCompletedFeatures
const finalParkingLot
const totalFeatures
const completedFeatures
const overallCompletionRate
const developOverallModelTask
const buildFeaturesListTask
const planByFeatureTask
const designByFeatureTask
const buildByFeatureTask
const generateParkingLotTask
const taskArgs
const svg
const completed
const total
const percentage

// Functions
function generateParkingLotSVG((featureSets, completedFeatures))

// Constants
const boxWidth
const boxHeight
const padding
const cols
const svgContent
const col
const row
const x
const y
const completed
const total
const percentage
const color

// Functions
function getStatusColor((percentage))
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/graph-of-thoughts.js`

- Size: 15861 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 509 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const graph
const rootNode
const rootId
const explorationQueue
const completeSolutionNodes
const scoreA
const scoreB
const current
const currentNode
const isSolution
const branches
const i
const branch
const newNodeId
const similarNode
const newNode
const evaluation
const parent
const unexploredSiblings
const sibling
const bestSolution
const leafNodes

// Functions
function getPathToNode((graph, nodeId))

// Constants
const path
const currentId
const node

// Functions
function findSimilarNode((graph, newThought, depth))

// Constants
const nodesAtDepth
const similarity

// Functions
function calculateSimilarity((thought1, thought2))

// Constants
const str1
const str2
const words1
const words2
const intersection
const union
const agentGenerateThoughtTask
const agentGenerateBranchesTask
const agentEvaluateThoughtTask
const agentEvaluateSolutionTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/hypothesis-driven-development/hypothesis-driven-development.js`

- Size: 36715 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1046 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const hypothesisResult
const experimentDesignResult
const mvpResult
const measurementPlanResult
const implementationResult
const experimentExecutionResult
const instrumentationValidation
const analysisResult
const decisionResult
const learningsResult
const formulateHypothesisTask
const designExperimentTask
const specifyMVPTask
const createMeasurementPlanTask
const implementMVPTask
const validateInstrumentationTask
const runExperimentTask
const analyzeResultsTask
const makeDecisionTask
const captureLearningsTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/impact-mapping/impact-mapping.js`

- Size: 34169 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 996 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const mappingStartTime
const goalDefinition
const actorIdentification
const actors
const impactResults
const result
const allImpacts
const allAssumptions
const deliverableResults
const actor
const result
const allDeliverables
const prioritization
const prioritizedDeliverables
const roadmap
const mapVisualization
const impactMap
const mermaidDiagram
const mappingEndTime
const sessionDurationMinutes
const goalDefinitionTask
const actorIdentificationTask
const impactAnalysisTask
const deliverableGenerationTask
const prioritizationTask
const mapVisualizationTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/jobs-to-be-done/jobs-to-be-done.js`

- Size: 28644 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 798 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const results
const jobDiscoveryResult
const loadedJobs
const mainJobs
const forcesResults
const loadedData
const mainJobs
const jobStoriesResults
const jobForces
const solutionMappingResult
const finalValidation
const jobDiscoveryTask
const forcesAnalysisTask
const jobStoryGenerationTask
const solutionMappingTask
const validateJTBDTask
const loadJobsTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/kanban/kanban.js`

- Size: 31952 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 889 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const boardVisualizationResult
const flowCycles
const currentBoard
const cumulativeMetrics
const cycleIdx
const cycleData
const wipManagementResult
const pullSystemResult
const flowMetricsResult
const replenishmentResult
const retrospectiveResult
const averageCycleTime
const averageLeadTime
const throughput
const boardVisualizationTask
const wipLimitManagementTask
const pullSystemTask
const flowMetricsTask
const replenishmentMeetingTask
const retrospectiveTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/plan-and-execute.js`

- Size: 6423 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 214 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const planResult
const executionResults
const i
const step
const stepResult
const allStepsSucceeded
const completedSteps
const agentPlanningTask
const executeStepTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/ralph.js`

- Size: 4280 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 146 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const iteration
const done
const results
const result
const success
const maxIterationsReached
const agentExecuteTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/rup/rup.js`

- Size: 92351 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 2565 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const inceptionResult
const iterationData
const visionResult
const businessCaseResult
const useCaseModelResult
const riskAssessmentResult
const projectPlanResult
const elaborationResult
const detailedUseCasesResult
const architectureResult
const prototypeResult
const riskMitigationResult
const refinedPlanResult
const constructionResult
const iterationPlanResult
const implementationResult
const integrationResult
const documentationResult
const reviewResult
const transitionResult
const betaTestingResult
const finalizationResult
const deploymentResult
const trainingResult
const tuningResult
const warrantyResult
const projectMetrics

// Functions
function executePhase((phaseName, iterationCount, ctx, iterationFunction))

// Constants
const iterations
const i
const iterationResult
const lastIteration
const createVisionDocumentTask
const createBusinessCaseTask
const createUseCaseModelTask
const assessRisksTask
const createProjectPlanTask
const refineUseCaseModelTask
const defineArchitectureTask
const buildArchitecturePrototypeTask
const mitigateRisksTask
const refineProjectPlanTask
const planIterationTask
const implementIterationTask
const integrateAndTestTask
const generateUserDocumentationTask
const reviewIterationTask
const conductBetaTestingTask
const finalizeProductTask
const deployToProductionTask
const conductUserTrainingTask
const tuneSystemTask
const setupWarrantySupportTask

// Functions
function calculateProjectMetrics((inception, elaboration, construction, transition, iterationsPerPhase))

// Constants
const totalIterations
const implementations
const totalModules
const totalLinesOfCode
const totalTests
const testsPassed
const testPassRate
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/scrum/scrum.js`

- Size: 41407 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions
- 1106 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const definitionOfDoneResult
const productBacklogResult
const sprintResults
const currentVelocity
const remainingBacklog
const sprintIdx
const sprintNumber
const refinementResult
const sprintPlanningResult
const sprintData
const workingDays
const dailyScrumResults
const day
const dailyScrumResult
const burndownResult
const sprintReviewResult
const retrospectiveResult
const velocityMetrics
const releaseBurndown
const establishDefinitionOfDoneTask
const createProductBacklogTask
const backlogRefinementTask
const sprintPlanningTask
const dailyScrumTask
const sprintReviewTask
const sprintRetrospectiveTask
const generateBurndownTask
const taskArgs
const workingDays
const idealBurndown
const actualBurndown
const day
const svg

// Functions
function calculateVelocityMetrics((sprintResults))

// Constants
const velocities
const totalCompletedStoryPoints
const averageVelocity
const totalCompletedItems
const firstHalf
const secondHalf
const firstHalfAvg
const secondHalfAvg
const velocityTrend

// Functions
function generateReleaseBurndown((initialBacklog, sprintResults))

// Constants
const burndownData
const remainingPoints

// Functions
function calculateStandardDeviation((values))

// Constants
const avg
const squareDiffs
const avgSquareDiff

// Functions
function generateBurndownSVG((idealBurndown, actualBurndown, sprintNumber))

// Constants
const width
const height
const padding
const maxPoints
const maxDays
const xScale
const yScale
const svg
const idealPath
const actualPath
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/self-assessment.js`

- Size: 8565 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 253 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const iteration
const qualityAcceptable
const results
const currentWork
const executionResult
const assessmentResult
const finalAssessment
const success
const agentExecuteTask
const agentSelfAssessTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/shape-up/shape-up.js`

- Size: 55714 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1569 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const validAppetites
const validPhases
const results
const appetiteResult
const breadboardResult
const rabbitHolesResult
const sketchesResult
const pitchResult
const pitchToEvaluate
const bettingEvaluation
const cyclePlan
const pitchToBuild
const planToExecute
const orientationResult
const scopeMappingResult
const cycleExecution
const checkIns
const totalCheckIns
const i
const weekNumber
const hillChartUpdate
const qaResult
const circuitBreakerCheck
const coolDownResult
const allPhasesComplete
const shipped
const completionRate
const defineAppetiteTask
const breadboardingTask
const identifyRabbitHolesTask
const fatMarkerSketchesTask
const writePitchTask
const bettingTableEvaluationTask
const createCyclePlanTask
const getOrientedTask
const mapScopesTask
const executeCycleTask
const updateHillChartTask
const qaAndIntegrationTask
const circuitBreakerCheckTask
const coolDownTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/spec-driven-development.js`

- Size: 18092 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 469 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const constitutionResult
const specificationResult
const clarificationResult
const planResult
const constitutionCheckResult
const revisedPlan
const tasksResult
const analysisResult
const implementationResults
const phaseIdx
const phase
const waves
const waveIdx
const wave
const waveResults
const task
const implResult
const checklistResult
const validationResult
const failedTasks
const userStoryValidation

// Functions
function organizeTasksIntoWaves((phaseTaskIds, allTasks))

// Constants
const waves
const remaining
const completed
const wave
const task
const canRun
const establishConstitutionTask
const createSpecificationTask
const clarifySpecificationTask
const createTechnicalPlanTask
const validateConstitutionTask
const breakIntoTasksTask
const analyzeConsistencyTask
const implementTaskTask
const generateChecklistTask
const validateChecklistTask
const validateUserStoriesTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/spec-kit-brownfield.js`

- Size: 7490 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 142 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const analysisResult
const constitutionResult
const specificationResult
const planResult
const integrationValidation
const tasksResult
const analyzeExistingCodebaseTask
const inferConstitutionTask
const specifyBrownfieldFeatureTask
const planBrownfieldImplementationTask
const validateIntegrationStrategyTask
const breakBrownfieldIntoTasksTask
const validateExistingConstitutionTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/spec-kit-constitution.js`

- Size: 3648 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 75 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const contextResult
const draftResult
const finalResult
const gatherConstitutionContextTask
const draftConstitutionTask
const finalizeConstitutionTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/spec-kit-quality-checklist.js`

- Size: 2953 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 62 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const checklistResult
const validationResult
const generateQualityChecklistTask
const validateAgainstChecklistTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/spiral-model/spiral-model.js`

- Size: 51159 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 1205 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const spiralResults
const cumulativeCost
const unresolvedRisks
const prototypes
const currentSystem
const converged
const spiralIdx
const spiralNumber
const spiralStartCost
const planningResult
const riskAnalysisResult
const criticalRisks
const highRisks
const engineeringResult
const evaluationResult
const spiralData
const spiralVisualization
const finalMetrics
const planningPhaseTask
const riskAnalysisTask
const engineeringPhaseTask
const evaluationPhaseTask
const spiralTrackingTask
const taskArgs
const spiralDiagram
const riskHeatmap
const trackingMetrics

// Functions
function calculateFinalMetrics((spiralResults, cumulativeCost, prototypes))

// Constants
const totalRisksIdentified
const risksResolved
const totalComponents
const totalTestsPassed
const averageCustomerSatisfaction
const integrationComplete

// Functions
function generateSpiralDiagramSVG((spirals, maxCost))

// Constants
const width
const height
const centerX
const centerY
const maxRadius
const svg
const phaseLabels
const phaseAngles
const rad
const labelRadius
const x
const y
const phases
const anglePerPhase
const spiralNumber
const startRadius
const endRadius
const phase
const startAngle
const endAngle
const radius
const nextRadius
const x1
const y1
const x2
const y2
const largeArcFlag
const labelAngle
const labelRadius
const labelX
const labelY

// Functions
function generateRiskHeatmapSVG((spirals))

// Constants
const width
const height
const padding
const cellWidth
const cellHeight
const svg
const categories
const colors
const y
const x
const risks
const count
const opacity
const x
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/state-machine-orchestration.js`

- Size: 21362 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 655 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const states
const currentState
const transitionCount
const stateHistory
const transitionLog
const taskContext
const enterState
const entry
const exitState
const executeEntryAction
const executeExitAction
const transition
const allowed
const trans
const previousState
const evaluateTransitionGuard
const validTransitions
const allowedStates
const maxRefinementAttempts
const planResult
const executionResult
const validationResult
const refinementResult
const agentPlanningTask
const agentExecutionTask
const agentValidationTask
const agentRefinementTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/top-down.js`

- Size: 18216 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 577 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const architecture
const decompositionTree
const allComponents
const currentLevel
const componentsAtCurrentLevel
const nextLevelComponents
const decomposition
const subcompNode
const implementationOrder
const implementationResults
const dependencies
const dependencyComponents
const implementation
const integrationResults
const level
const componentsAtLevel
const integrationTest
const stubReplacements
const stubbedComponents
const replacement
const validation

// Functions
function getDepthFirstOrder((root))

// Constants
const order

// Functions
function traverse((node))
function getComponentCountByLevel((components, maxLevel))

// Constants
const counts
const i
const agentDesignArchitectureTask
const agentDecomposeComponentTask
const agentImplementComponentTask
const agentIntegrationTestTask
const agentReplaceStubTask
const agentValidateSystemTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/v-model/v-model.js`

- Size: 57062 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 1 functions
- 1543 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const processStartTime
const requirementsPhase
const requirements
const acceptanceTestDesign
const systemDesignPhase
const systemDesign
const systemTestDesign
const architecturePhase
const architecture
const integrationTestDesign
const moduleDesignResults
const result
const moduleDesigns
const unitTestDesigns
const allUnitTests
const implementationPhase
const implementation
const executeTestsPhase
const testResults
const allTestsPassed
const traceabilityMatrix
const traceabilityPhase
const processEndTime
const durationMinutes
const requirementsWithAcceptanceTask
const systemDesignWithSystemTestTask
const architectureWithIntegrationTestTask
const moduleDesignWithUnitTestTask
const implementationTask
const executeTestsTask
const traceabilityMatrixTask
```

### File: `plugins/babysitter/skills/babysit/process/methodologies/waterfall/waterfall.js`

- Size: 58608 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 2 functions
- 1409 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const requirementsResult
const designResult
const implementationResult
const testingResult
const deploymentResult
const maintenanceResult
const projectMetrics
const requirementsGatheringTask
const systemDesignTask
const implementationTask
const testingPhaseTask
const deploymentTask
const maintenancePlanningTask

// Functions
function calculateProjectMetrics((requirements, design, implementation, testing, deployment, maintenance))

// Constants
const totalRequirements
const requirementsCovered
const completionRate
```

### File: `plugins/babysitter/skills/babysit/process/tdd-quality-convergence.js`

- Size: 19487 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 593 lines (0 code)

**Signatures:**

```javascript
// Functions
function process((inputs, ctx))
function process((inputs, ctx))

// Constants
const inputs
const planningResult
const iteration
const currentQuality
const converged
const iterationResults
const testsResult
const testRunResult
const implementationResult
const finalTestResult
const coverageResult
const qualityScore
const finalTestResult
const finalReview
const agentPlanningTask
const writeTestsTask
const runTestsTask
const implementCodeTask
const coverageCheckTask
const lintCheckTask
const typeCheckTask
const securityCheckTask
const agentQualityScoringTask
const integrationTestTask
const agentFinalReviewTask
```

### File: `test-dispatcher.js`

- Size: 680 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 27 lines (0 code)

**Signatures:**

```javascript
// Constants
const require

// Functions
function test(())

// Constants
const result
```

### File: `test-project-root.js`

- Size: 465 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Signatures:**

```javascript
// Constants
const path
const runDir
const projectRoot
```

### File: `video/remotion.config.ts`

- Size: 117 bytes
- Modified: 2026-02-15 10:31:50 UTC


### File: `video/src/Root.tsx`

- Size: 420 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 18 lines (0 code)

**Signatures:**

```typescript
// Constants
const Root : React.FC
```

### File: `video/src/Video.tsx`

- Size: 2280 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 66 lines (0 code)

**Signatures:**

```typescript
// Constants
const SCENES
const Video : React.FC
```

### File: `video/src/components/ApprovalModal.tsx`

- Size: 4086 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 interfaces
- 148 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ApprovalModalProps

// Constants
const frame
const scale
const opacity
const isApproved
const fadeOut
```

### File: `video/src/components/Logo.tsx`

- Size: 1898 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 interfaces
- 84 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface LogoProps

// Constants
const frame
const opacity
const scale
const glowIntensity
```

### File: `video/src/components/NeonText.tsx`

- Size: 1039 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions, 1 interfaces
- 50 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface NeonTextProps

// Constants
const NeonText : React.FC<NeonTextProps>
const frame
const opacity
const scale
```

### File: `video/src/components/Pipeline.tsx`

- Size: 5113 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 2 functions, 1 interfaces
- 210 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface PipelineStepProps

// Constants
const frame
const opacity
const scale
const getStatusColor
const getStatusGlow
const statusColor
const statusGlow
const frame
```

### File: `video/src/components/ProgressBar.tsx`

- Size: 3805 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 2 functions, 1 interfaces
- 160 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface ProgressBarProps

// Constants
const getColorForProgress : string
const getGlowForProgress : string
const frame
const animatedProgress
const barColor
const glow
const frame
const progress
const barColor
const glow
```

### File: `video/src/components/Terminal.tsx`

- Size: 2177 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 interfaces
- 100 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface TerminalProps

// Constants
const frame
const opacity
const scale
```

### File: `video/src/scenes/Breakpoints.tsx`

- Size: 7539 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 254 lines (0 code)

**Signatures:**

```typescript
// Constants
const frame
const steps
const getStepDone
const modalDelay
const approveFrame
const isApproved
const titleOpacity
const finalOpacity
const pausedOpacity
const pausedFadeOut
const isDone
const isBreakpoint
const isPaused
const stepOpacity
const borderColor
const glow
```

### File: `video/src/scenes/CTA.tsx`

- Size: 2611 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Signatures:**

```typescript
// Constants
const frame
const installOpacity
const installScale
const urlOpacity
```

### File: `video/src/scenes/CompoundingError.tsx`

- Size: 7436 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 4 functions
- 256 lines (0 code)

**Signatures:**

```typescript
// Constants
const STEPS
const getColorForPercentage : string
const getGlowForPercentage : string
const frame
const getStepProgress
const stepStart
const progress
const getCurrentQuality
const quality
const i
const progress
const stepStart
const stepEnd
const quality
const qualityColor
const qualityGlow
const showFinalMessage
const finalOpacity
const progress
const isActive
const isComplete
const stepPct
const stepColor
const stepGlow
```

### File: `video/src/scenes/Hook.tsx`

- Size: 2324 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Signatures:**

```typescript
// Constants
const frame
const codeLines
const visibleLines
const textOpacity
const textScale
```

### File: `video/src/scenes/QualityConvergence.tsx`

- Size: 8219 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 274 lines (0 code)

**Signatures:**

```typescript
// Constants
const STEPS
const frame
const titleOpacity
const getStepState
const stepStart
const localFrame
const retries
const framesPerRetry
const retryIndex
const percentage
const isComplete
const cliOpacity
const finalOpacity
const state
const stepStart
const isVisible
const stepOpacity
const borderColor
const glow
```

### File: `video/src/scenes/Resumability.tsx`

- Size: 8664 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 1 functions
- 293 lines (0 code)

**Signatures:**

```typescript
// Constants
const frame
const isClosing
const isClosed
const isReopening
const terminalScale
const closedMessageOpacity
const resumeMessageOpacity
const currentStep
const steps
```

### File: `video/src/styles/theme.ts`

- Size: 1253 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Signatures:**

```typescript
// Constants
const colors
const fonts
const glows
const timing
const dimensions
```

### File: `packages/vscode-extension/src/test/suite/index.ts`

- Size: 674 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 26 lines (0 code)

**Signatures:**

```typescript
// Functions
function run() : Promise<void>
function run() : Promise<void>

// Constants
const mocha
const testsRoot
```

### File: `packages/sdk/src/cli/__tests__/cliMain.test.ts`

- Size: 6861 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 23 functions
- 195 lines (0 code)

**Signatures:**

```typescript
// Constants
const buildEffectIndexMock
const readRunMetadataMock
const commitEffectResultMock
const logSpy : MockInstance<[message?: any, ...optionalParams: any[]], void>
const errorSpy : MockInstance<[message?: any, ...optionalParams: any[]], void>
const cli
const helpText
const cli
const exitCode
const cli
const exitCode
const cli
const exitCode
const cli
const exitCode
const payload
const cli
const exitCode
const cli
const exitCode

// Functions
function mockRunMetadata()
function nodeEffectRecord(effectId: string, overrides: Partial<EffectRecord> = {}) : EffectRecord

// Constants
const effectDir

// Functions
function mockEffectIndex(records: EffectRecord[])
```

### File: `packages/sdk/src/cli/__tests__/cliRuns.test.ts`

- Size: 24159 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 75 functions
- 692 lines (0 code)

**Signatures:**

```typescript
// Constants
const realReadRunMetadata
const runsRoot : string
const logSpy : ReturnType<typeof vi.spyOn>
const errorSpy : ReturnType<typeof vi.spyOn>
const entryFile
const inputsPath
const cli
const exitCode
const runDir
const metadata
const journal
const entryFile
const cli
const exitCode
const payload
const runDir
const metadata
const entryFile
const cli
const exitCode
const line
const entryFile
const inputsPath
const cli
const exitCode
const payload
const cli
const exitCode

// Functions
function writeEntrypoint(relativePath: string, contents: string)

// Constants
const absolutePath

// Functions
function expectSingleRunDir() : Promise<string>

// Constants
const entries
const runDir

// Functions
function listRunDirs() : Promise<string[]>

// Constants
const entries

// Functions
function toPosixRelative(fromDir: string, target: string) : string

// Constants
const relative

// Functions
function readLastJsonLine(spy: ReturnType<typeof vi.spyOn>)

// Constants
const raw
const runsRoot : string
const cli : ReturnType<typeof createBabysitterCli>
const logSpy : ReturnType<typeof vi.spyOn>
const errorSpy : ReturnType<typeof vi.spyOn>
const runDir
const exitCode
const line
const runDir
const exitCode
const payload
const runId
const runDir
const exitCode
const payload
const runDir
const exitCode
const payload
const runDir
const exitCode
const line
const runDir
const exitCode
const line
const missingDir
const exitCode
const runDir
const exitCode
const line
const runDir
const exitCode
const line
const runDir
const exitCode
const header
const entries
const runDir
const exitCode
const payload
const runDir
const exitCode
const missingDir
const exitCode
const runDir
const iterationMetadata
const exitCode
const payload
const lastEvent
const runDir
const exitCode
const header
const runDir
const exitCode
const payload
const runDir
const exitCode
const line
const runDir
const exitCode
const payload
const runDir
const exitCode
const line
const runDir
const exitCode
const payload

// Functions
function createRunWithPendingEffects()

// Constants
const runDir

// Functions
function createRunWithHistory()

// Constants
const runDir

// Functions
function seedStateCache(
    runDir: string,
    options: { stateVersion?: number; pendingEffectsByKind?: Record<string, number> } = {}
  )
function createRunSkeleton(runId: string)
function appendRequestedEffect(runDir: string, effectId: string, kind: string, label: string)

// Constants
const refs

// Functions
function appendResolvedEffect(runDir: string, effectId: string)
function writeTaskFiles(runDir: string, effectId: string, kind: string)

// Constants
const taskDir
const taskDefPath
const inputsPath

// Functions
function readLastJson(spy: ReturnType<typeof vi.spyOn>)

// Constants
const raw

// Functions
function linesFrom(spy: ReturnType<typeof vi.spyOn>)
function findSingleLine(spy: ReturnType<typeof vi.spyOn>, predicate: (line: string) => boolean)

// Constants
const line

// Functions
function collectPrefixed(spy: ReturnType<typeof vi.spyOn>, prefix: string)
function hasLineContaining(spy: ReturnType<typeof vi.spyOn>, needle: string)
function expectStateCacheMissing(runDir: string)

// Constants
const stateFile
```

### File: `packages/sdk/src/cli/__tests__/cliTasks.test.ts`

- Size: 12860 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 37 functions
- 319 lines (0 code)

**Signatures:**

```typescript
// Constants
const buildEffectIndexMock
const readTaskDefinitionMock
const readTaskResultMock
const logSpy : ReturnType<typeof vi.spyOn>
const errorSpy : ReturnType<typeof vi.spyOn>
const statSpy : ReturnType<typeof vi.spyOn>
const secretEnvSnapshot : string | undefined
const cli
const exitCode
const cli
const exitCode
const payload
const cli
const exitCode
const entries
const cli
const exitCode
const payload
const cli
const exitCode
const cli
const exitCode
const payload
const cli
const exitCode
const payload
const cli
const exitCode
const payload
const cli
const exitCode
const payload
const cli
const exitCode
const payload
const cli
const exitCode
const cli
const exitCode

// Functions
function effectRecord(effectId: string, overrides: Partial<EffectRecord> = {}) : EffectRecord

// Constants
const effectDir
const base : EffectRecord
const record : EffectRecord

// Functions
function mockEffectIndex(records: EffectRecord[])
function collectLines(spy: ReturnType<typeof vi.spyOn>)
function collectEntries(spy: ReturnType<typeof vi.spyOn>)
function expectEntries(spy: ReturnType<typeof vi.spyOn>, count: number)
function expectLogContaining(spy: ReturnType<typeof vi.spyOn>, substring: string)

// Constants
const lines

// Functions
function expectLogNotContaining(spy: ReturnType<typeof vi.spyOn>, substring: string)

// Constants
const lines

// Functions
function readLastJson(spy: ReturnType<typeof vi.spyOn>)

// Constants
const raw
```

### File: `packages/sdk/src/runtime/__tests__/commitEffectResult.test.ts`

- Size: 7352 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 15 functions
- 237 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string
const sampleTask : DefinedTask<{ value: number }, { doubled: number }>
const effect
const resolvedRecord
const loggerEntries : Array<Record<string, unknown>>
const effect
const metrics : Record<string, unknown>[]
const effect
const effect
const metrics : Record<string, unknown>[]
const stdoutPath
const stderrPath
const index
const record
const registryRecord
const effect
const metrics : Record<string, unknown>[]

// Functions
function requestSampleEffect()

// Constants
const context
```

### File: `packages/sdk/src/runtime/__tests__/createReplayEngine.test.ts`

- Size: 1923 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 7 functions
- 52 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string
const metadataPath
const metadata
const journalDir
```

### File: `packages/sdk/src/runtime/__tests__/createRun.test.ts`

- Size: 3065 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 5 functions
- 90 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string
const entryFile
const result
const metadata
const journal
const entryFile
const result
const metadata
const inputs
const journal
```

### File: `packages/sdk/src/runtime/__tests__/deterministicHarness.test.ts`

- Size: 4303 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions, 1 interfaces
- 131 lines (0 code)

**Signatures:**

```typescript
// Constants
const PROCESS_FILENAME
const fixtureRoot : string
const processPath
const first
const second

// Traits/Interfaces
interface RunArtifacts

// Functions
function executeDeterministicRun(processPath: string, runId: string) : Promise<RunArtifacts>

// Constants
const harness
const waitingSnapshot
const firstRequest
const journalInvocationKey
const completion
const snapshot
const invocationKeys

// Functions
function writeResumeProcess(root: string)

// Constants
const processPath
const contents

// Functions
function createFailingResolver() : FakeActionResolver

// Constants
const thrown

// Functions
function createTaskResolver() : FakeActionResolver

// Constants
const value
```

### File: `packages/sdk/src/runtime/__tests__/effectActions.contract.test.ts`

- Size: 4335 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 133 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string
const processDir
const processPath
const sleepIso
const runId
const iteration
const actions
const parallelGroupIds
const sleepAction
const sanitized

// Functions
function sanitizeActions(actions: EffectAction[])

// Constants
const parallelMapping
const groupCounter
const mapParallelId
const schedulerHints

// Functions
function scrubRef(ref: string | undefined, effectId: string, index: number)
```

### File: `packages/sdk/src/runtime/__tests__/effectIndex.test.ts`

- Size: 6457 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 17 functions
- 184 lines (0 code)

**Signatures:**

```typescript
// Constants
const runDir

// Functions
function makeEvent(seq: number, type: string, data: Record<string, unknown>) : JournalEvent

// Constants
const filename
const events
const index
const events
const events
const first
const second
const runError
const badEvent
const badEvent
const runError
const orphan
const runError
const events
const runError
const runDir
const journalDir
const badPath
```

### File: `packages/sdk/src/runtime/__tests__/exceptions.test.ts`

- Size: 6442 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 214 lines (0 code)

**Signatures:**

```typescript
// Constants
const taskDef : TaskDef
const effectAction : EffectAction

// Functions
function serializeContractError(error: BabysitterRuntimeError)

// Constants
const error
const error
const extraAction : EffectAction
const error
const error
```

### File: `packages/sdk/src/runtime/__tests__/intrinsics.behaviors.test.ts`

- Size: 7233 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 28 functions
- 175 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string
const now
const context
const target
const earlyContext
const lateContext
const target
const context
const action
const target
const firstContext
const pendingContext
const action
const requestedAt
const context
const action
const context
const action
const context
const action
const context
const action
const context
const payload
const action
```

### File: `packages/sdk/src/runtime/__tests__/orchestrateIteration.integration.test.ts`

- Size: 4079 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 11 functions
- 126 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string

// Functions
function writeProcessFile(dir: string, filename: string)

// Constants
const filePath
const contents
const processDir
const processPath
const firstIteration
const action
const secondIteration
const processDir
const processPath
const runId
const metrics : Record<string, unknown>[]
const logger
const entry
const waitingResult
const completion
const replayMetrics
```

### File: `packages/sdk/src/runtime/__tests__/parallel.test.ts`

- Size: 2325 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 72 lines (0 code)

**Signatures:**

```typescript
// Constants
const taskDef : TaskDef

// Functions
function makeAction(effectId: string) : EffectAction

// Constants
const actionA
const actionB
const actionC
const thunks
const pending
const groupIds
const summaries
```

### File: `packages/sdk/src/runtime/__tests__/processContext.test.ts`

- Size: 9338 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 41 functions
- 285 lines (0 code)

**Signatures:**

```typescript
// Functions
function effectIndexStub() : EffectIndex

// Constants
const sampleTaskDef : TaskDef

// Functions
function makeAction(id: string) : EffectAction

// Constants
const delay
const createProcessContext
const createProcessContext
const createProcessContext
const createProcessContext
const createProcessContext
const createProcessContext
const createProcessContext
const createProcessContext
const createProcessContext
const actionA
const actionB
const actionC
const parallelError
const groupIds
const createProcessContext
const actionA
const actionB
const parallelError
const groupIds
const createProcessContext
const values
```

### File: `packages/sdk/src/runtime/__tests__/replayCursor.test.ts`

- Size: 684 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 21 lines (0 code)

**Signatures:**

```typescript
// Constants
const cursor
const cursor
const i
```

### File: `packages/sdk/src/runtime/__tests__/taskIntrinsic.test.ts`

- Size: 7568 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 12 functions
- 247 lines (0 code)

**Signatures:**

```typescript
// Constants
const sampleTask : DefinedTask<{ value: number }, number>
const tmpRoot : string

// Functions
function createRun(runId = "run-task")
function buildContext(runDir: string, runId: string) : Promise<TaskIntrinsicContext>

// Constants
const effectIndex
const replayCursor
const context : TaskIntrinsicContext
const context
const requestedEffectId : string | undefined
const pendingContext
const replayCtx
const context
const effectId
const replayCtx
const context
const effectId
const largeValue
const replayCtx
const context
const buildSpy
const blobRef
const relPath
const definedTask : DefinedTask<{ value: number }, number>
const capturedError : unknown
const effectId
const registryRecord
const refreshedIndex
const indexed
```

### File: `packages/sdk/src/runtime/__tests__/testHelpers.ts`

- Size: 1287 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions, 2 interfaces
- 45 lines (0 code)

**Signatures:**

```typescript
// Traits/Interfaces
interface TestRunInfo
interface TestRunInfo

// Functions
function createTestRun(tmpRoot: string, runId = `run-${Date.now()}`) : Promise<TestRunInfo>
function createTestRun(tmpRoot: string, runId = `run-${Date.now()}`) : Promise<TestRunInfo>

// Traits/Interfaces
interface BuildTaskContextOptions
interface BuildTaskContextOptions

// Functions
function buildTaskContext(
  runDir: string,
  runId: string,
  options?: BuildTaskContextOptions
) : Promise<TaskIntrinsicContext>
function buildTaskContext(
  runDir: string,
  runId: string,
  options?: BuildTaskContextOptions
) : Promise<TaskIntrinsicContext>

// Constants
const effectIndex
const replayCursor
```

### File: `packages/sdk/src/runtime/replay/__tests__/stateCache.test.ts`

- Size: 5217 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions
- 157 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string

// Functions
function createTestRun(runId = `run-${Date.now()}`) : Promise<string>

// Constants
const runDir
const snapshot
const runDir
const stateFile
const repaired
const normalized
const base
const other
```

### File: `packages/sdk/src/storage/__tests__/storage.test.ts`

- Size: 4579 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 124 lines (0 code)

**Signatures:**

```typescript
// Constants
const tmpRoot : string
const runJson
const journalDir
const files
const events
const contents
const artifactsManifest
const blobEntry
const usage
const orphaned
```

### File: `packages/sdk/src/tasks/__tests__/batching.test.ts`

- Size: 3074 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 9 functions
- 90 lines (0 code)

**Signatures:**

```typescript
// Constants
const baseTaskDef : TaskDef

// Functions
function makeAction(effectId: string, overrides: Partial<EffectAction> = {}) : EffectAction

// Constants
const actionA
const actionB
const actionC
const batch
const groupIds
const action
const summary
const batch
const payload
```

### File: `packages/sdk/src/tasks/__tests__/context.test.ts`

- Size: 3758 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 15 functions
- 105 lines (0 code)

**Signatures:**

```typescript
// Constants
const EFFECT_ID
const runDir : string
const buildCtx
const ctx
const ctx
const ctx
const ref
const filePath
const onDisk
const sameRef
const ctx
const ref
const ctx
const textRef
const textContents
const buffer
const binRef
const stored
const ctx
const ctx
const normalized
const ctx
const absolute

// Functions
function resolveRef(runDir: string, ref: string) : string

// Constants
const segments
```

### File: `packages/sdk/src/tasks/__tests__/defineTask.test.ts`

- Size: 3443 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 27 functions
- 110 lines (0 code)

**Signatures:**

```typescript
// Constants
const ctxCounter
const fakeCtx : TaskBuildContext
const suffix
const labels : string[]
const defined
const defined
const defined
const firstCtx
const secondCtx
const first
const second
const defined
const ctx
const record
```

### File: `packages/sdk/src/tasks/__tests__/kinds.test.ts`

- Size: 6005 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 23 functions, 1 interfaces
- 157 lines (0 code)

**Signatures:**

```typescript
// Constants
const helper
const ctx
const def
const helper
const ctx
const def
const helper
const ctx
const def
const helper
const ctx
const def
const helper
const ctx
const def
const expectedLabel

// Functions
function createTestBuildContext(options: Partial<TestContextOptions> = {}) : TaskBuildContext

// Constants
const effectId
const runDir
const label
const labels

// Traits/Interfaces
interface TestContextOptions
```

### File: `packages/sdk/src/tasks/__tests__/registry.test.ts`

- Size: 3093 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions, 1 types
- 103 lines (0 code)

**Signatures:**

```typescript
// Constants
const TASK_JSON_REF
const registry : TaskRegistry
const first
const nodePending
const labelFiltered
const before
const after

// Type Aliases
type EffectInput

// Functions
function recordEffect(registry: TaskRegistry, input: EffectInput)
```

### File: `packages/sdk/src/tasks/__tests__/serializer.test.ts`

- Size: 3447 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 6 functions
- 97 lines (0 code)

**Signatures:**

```typescript
// Constants
const EFFECT_ID
const runDir : string
const onDisk
const bigPayload
const absoluteRef
const blob
const hugeResult
const serializedResult
const blobContents
```

### File: `packages/sdk/src/testing/__tests__/parallelHarness.test.ts`

- Size: 3584 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 8 functions
- 98 lines (0 code)

**Signatures:**

```typescript
// Constants
const PROCESS_FILENAME
const fixtureRoot : string
const processPath
const harness
const result
const pendingBranch
const firstIteration
const groupIds

// Functions
function writeParallelProcess(root: string)

// Constants
const processPath
const contents

// Functions
function resolveOnlyAlphaBranch() : FakeActionResolver

// Constants
const handled
const metadata
```

### File: `packages/sdk/src/testing/__tests__/runHarness.test.ts`

- Size: 4417 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 10 functions
- 120 lines (0 code)

**Signatures:**

```typescript
// Functions
function buildUlidSequence(count: number)

// Constants
const suffix
const seeds
const harness
const resolutionValues : number[]
const result
const value
const logEffectIds
const seeds
const harness
const result
const value
const harness
const value

// Functions
function createCounterHarness(runId: string, initialValue: number, ulidPreset?: string[])
```

### File: `packages/vscode-extension/src/test/runTest.ts`

- Size: 1509 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 54 lines (0 code)

**Signatures:**

```typescript
// Functions
function isHeadlessRun() : boolean
function makeTempDir(prefix: string) : string
function main() : Promise<void>

// Constants
const extensionDevelopmentPath
const extensionTestsPath
const fixtureWorkspacePath
const tempRoot
const testWorkspacePath
const headless
const launchArgs
```

### File: `packages/vscode-extension/src/test/suite/constants.ts`

- Size: 318 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Signatures:**

```typescript
// Constants
const packageJson
const EXTENSION_ID
```

### File: `packages/vscode-extension/src/test/suite/dispatch.test.ts`

- Size: 3904 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 98 lines (0 code)

**Signatures:**

```typescript
// Functions
function createOBinaryShim(tempDir: string, argsOutputPath: string) : string

// Constants
const shimScriptPath
const runId
const shimCmdPath
const shimShPath
const ext
const workspaceRoot
const tempDir
const argsOutputPath
const shimPath
const cfg
const prompt
const previousHeadless
const result
const argsJson
```

### File: `packages/vscode-extension/src/test/suite/extension.test.ts`

- Size: 953 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 3 functions
- 26 lines (0 code)

**Signatures:**

```typescript
// Constants
const ext
const api
const ext
const commands
```

### File: `packages/vscode-extension/src/test/suite/promptBuilderUi.test.ts`

- Size: 7974 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 13 functions, 1 classes
- 215 lines (0 code)

**Signatures:**

```typescript
// Structs/Classes
class MemoryMemento

// Functions
function makeWebview() : { webview: PromptBuilderWebviewLike; messages: unknown[] }

// Constants
const messages : unknown[]
const workspaceRoot
const memento
const makeWebview
const bridge
const workspaceRoot
const memento
const makeWebview
const bridge
const inside
const outside
const handled
const msg
const workspaceRoot
const memento
const makeWebview
const bridge
const hydrated
const workspaceRoot
const memento
const makeWebview
const bridge
const workspaceRoot
const memento
const makeWebview
const bridge
const handled
const workspaceRoot
const memento
const makeWebview
const bridge
const handled
const workspaceRoot
const memento
const makeWebview
const bridge
const handled
const workspaceRoot
const memento
const makeWebview
const bridge
const handledEmpty
const handled
const workspaceRoot
const memento
const makeWebview
const bridge
const handled
```

### File: `packages/vscode-extension/src/test/suite/resume.test.ts`

- Size: 4296 bytes
- Modified: 2026-02-15 10:31:49 UTC


**Structure:**
- 4 functions
- 108 lines (0 code)

**Signatures:**

```typescript
// Functions
function createOBinaryShim(tempDir: string, argsOutputPath: string) : string

// Constants
const shimScriptPath
const shimCmdPath
const shimShPath
const ext
const workspaceRoot
const tempDir
const argsOutputPath
const shimPath
const cfg
const runId
const prompt
const result
const argsJson
const runsRoot
const runs
const resumed
```

### File: `scripts/bump-version.mjs`

- Size: 6909 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Signatures:**

```javascript
// Constants
const run
const bumpVersion
const major
const packageManifests
const pluginManifests
const manifests
const pluginManifestData
const rootManifest
const currentVersion
const lastTag
const logRange
const logCmd
const commits
const bumpTarget
const newVersion
const currentPluginVersion
const newPluginVersion
const marketplacePath
const marketplaceData
const babysitterPlugin
const currentMarketplaceVersion
const newMarketplaceVersion
const lockPath
const lock
const extensionWorkspaceKey
const sdkWorkspaceKey
const breakpointsWorkspaceKey
const babysitterWorkspaceKey
const extensionManifest
const extensionName
const extensionNodeModulesKey
const sdkManifest
const sdkName
const sdkNodeModulesKey
const breakpointsManifest
const breakpointsName
const breakpointsNodeModulesKey
const babysitterManifest
const babysitterName
const babysitterNodeModulesKey
const changelogPath
const changelog
const unreleasedPattern
const matches
const unreleasedBody
const isPlaceholder
const releaseBody
const placeholder
const isoDate
const replacement
const updatedChangelog
```

### File: `scripts/check-package-metadata.cjs`

- Size: 1251 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 2 functions
- 50 lines (0 code)

**Signatures:**

```javascript
// Constants
const fs
const path
const EXPECTED_REPOSITORY_URL
const EXPECTED_USER_GUIDE_LINK
const repoRoot
const packageJsonPath
const readmePath

// Functions
function fail((message))
function readJson((filePath))

// Constants
const packageJson
const repositoryUrl
const readmeContent
```

### File: `scripts/release-notes.mjs`

- Size: 478 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Signatures:**

```javascript
// Constants
const changelogPath
const changelog
const pattern
const match
const body
```

### File: `scripts/update-vscodeignore-deps.mjs`

- Size: 2745 bytes
- Modified: 2026-02-15 10:31:50 UTC


**Structure:**
- 4 functions
- 89 lines (0 code)

**Signatures:**

```javascript
// Constants
const __filename
const __dirname
const repoRoot
const extensionRoot
const vscodeIgnorePath
const startMarker
const endMarker
const excludedScopes

// Functions
function isExcluded((name))

// Constants
const scope

// Functions
function collectDependencies((tree))

// Constants
const result
const stack
const deps

// Functions
function buildAllowlist(())

// Constants
const raw
const data
const names
const filtered
const patterns
const scope

// Functions
function updateFile((patterns))

// Constants
const contents
const startIndex
const endIndex
const before
const after
const block
const updated
const patterns
```
