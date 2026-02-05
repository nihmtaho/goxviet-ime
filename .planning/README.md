# 📋 GoxViet Planning Repository

Welcome to the planning directory for GoxViet IME project. This directory contains structured project roadmaps, milestones, and task management in machine-readable JSON format.

## 📂 Structure

```
.planning/
├── README.md (this file)
├── ROADMAP_SUMMARY_ISSUES_54_55.md
├── roadmap_issue_55_tone_repositioning_20260205_193900.json
├── roadmap_issue_54_zen_browser_20260205_193900.json
└── [Additional roadmaps as needed]
```

## 🎯 Current Roadmaps

### Active Roadmaps (v2.1.0)

| ID | Title | Status | Timeline | Link |
|----|-------|--------|----------|------|
| #55 | Tone Mark Positioning Fix | 📍 Planned | Feb 10-28 | `roadmap_issue_55_tone_repositioning_20260205_193900.json` |
| #54 | Character Duplication on Zen Browser | 📍 Planned | Feb 10-25 | `roadmap_issue_54_zen_browser_20260205_193900.json` |

**Total Budget**: $4,600 USD  
**Total Tasks**: 24  
**Estimated Effort**: ~77 hours  

---

## 📖 How to Use

### Reading Roadmaps

1. **Quick Summary**: Start with `ROADMAP_SUMMARY_ISSUES_54_55.md`
   - Overview of both issues
   - Timeline visualization
   - Task breakdown
   - Success criteria

2. **Detailed Roadmap**: Open corresponding JSON file
   - Full hierarchical structure (phases → milestones → tasks)
   - Task dependencies and effort estimates
   - Budget allocation
   - Resource assignments

### Updating Progress

To update task status:

1. Open the roadmap JSON file
2. Find the specific phase/milestone/task
3. Update fields:
   - `"status"`: "not_started" → "in_progress" → "completed"
   - `"completion_percentage"`: 0 → 50 → 100
   - `"actual_hours"`: Insert actual time spent
4. Save and commit to repository

Example:
```json
{
  "task_id": "task_001",
  "status": "in_progress",  // Updated from "not_started"
  "completion_percentage": 45,  // 45% complete
  "actual_hours": 2  // 2 hours spent so far
}
```

### Adding New Roadmaps

Use naming convention: `roadmap_[FEATURE_NAME]_[YYYYMMDD]_[HHMMSS].json`

Examples:
- `roadmap_dark_mode_feature_20260205_143000.json`
- `roadmap_performance_optimization_20260205_150000.json`

---

## 🏗️ Roadmap Format

Each roadmap follows this hierarchy:

```
Roadmap (metadata + timeline)
├── Phase 1 (major section)
│   ├── Milestone 1 (significant achievement)
│   │   ├── Task 1 (actionable work)
│   │   │   └── Subtask 1 (granular detail)
│   │   ├── Task 2
│   │   └── ...
│   ├── Milestone 2
│   └── ...
├── Phase 2
└── ...
```

### Core Fields

**Roadmap Level**
- `metadata`: Project info, owner, version
- `timeline`: start_date, end_date, duration
- `roadmap`: Contains all phases

**Phase Level**
- `phase_id`: Unique identifier
- `name`: Phase name
- `status`: "not_started" | "in_progress" | "completed"
- `priority`: "critical" | "high" | "medium" | "low"
- `milestones`: Array of milestone objects

**Milestone Level**
- `milestone_id`: Unique identifier
- `name`: Milestone name
- `status`: "not_started" | "in_progress" | "completed"
- `planned_date`: Target completion date
- `completion_percentage`: 0-100
- `tasks`: Array of task objects

**Task Level**
- `task_id`: Unique identifier
- `name`: Task name
- `type`: "research" | "development" | "testing" | "documentation" | etc.
- `status`: "not_started" | "in_progress" | "completed"
- `assigned_to`: Assignee username
- `estimated_hours`: Time estimate
- `actual_hours`: Time spent
- `dependencies`: Array of prerequisite task IDs
- `completion_percentage`: 0-100

---

## 📊 Metrics & Tracking

### Completion Calculation (Bottom-Up)
1. Task completion % (directly assigned)
2. Milestone completion % = average of task completions
3. Phase completion % = average of milestone completions
4. Overall completion % = average of phase completions

### Status Indicators
- 🟢 **GREEN**: On schedule, under budget
- 🟡 **YELLOW**: Slightly behind, monitor closely
- 🔴 **RED**: Blocked or critical issues

### Budget Tracking
- `allocated`: Budget reserved (USD)
- `spent`: Actual expenses
- Utilization: (spent / allocated) × 100%

---

## 🔄 Update Frequency

| Item | Frequency | Owner |
|------|-----------|-------|
| Task Status | Daily | Assignee |
| Milestone Status | Weekly | Phase Owner |
| Overall Roadmap | Weekly | Project Owner |
| Budget Review | Weekly | Project Owner |
| Risk Assessment | As needed | Project Owner |

---

## 📞 Contact

- **Project Owner**: nihmtaho
- **Last Updated**: 2026-02-05
- **Repository**: https://github.com/nihmtaho/goxviet-ime

---

## 📝 Notes

- All times are in UTC
- Dates use ISO 8601 format (YYYY-MM-DD)
- Task dependencies prevent circular references
- Budgets are in USD currency
- JSON files are machine-readable and version-controllable

---

**Status**: Active Planning  
**Version**: 1.0.0
