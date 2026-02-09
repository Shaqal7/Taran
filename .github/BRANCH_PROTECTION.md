# Branch Protection Policy

## Main Branch Protection

The `main` branch is protected to ensure code quality and prevent accidental or unauthorized changes.

### Rules

1. **No Direct Pushes**: Direct pushes to the `main` branch are **blocked**
2. **Pull Request Required**: All changes must go through a Pull Request (PR)
3. **Code Review Required**: At least one approving review is required before merging
4. **No Force Push**: Force pushes to `main` are not allowed
5. **No Deletion**: The `main` branch cannot be deleted

### Workflow

To contribute to this repository:

1. Create a new branch from `main`:
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit them:
   ```bash
   git add .
   git commit -m "Description of changes"
   ```

3. Push your branch to GitHub:
   ```bash
   git push origin feature/your-feature-name
   ```

4. Create a Pull Request on GitHub targeting the `main` branch

5. Wait for review and approval

6. Once approved, merge the PR (the branch will be automatically deleted after merge)

### Configuration

Branch protection is configured through:
- `.github/settings.yml` - Automated configuration (requires GitHub Settings App)
- GitHub repository settings - Manual configuration via web interface

### Manual Setup

If the GitHub Settings App is not installed, repository administrators should configure branch protection manually:

1. Go to repository **Settings** > **Branches**
2. Add a branch protection rule for `main`
3. Enable the following:
   - ✅ Require a pull request before merging
     - ✅ Require approvals (at least 1)
     - ✅ Dismiss stale pull request approvals when new commits are pushed
   - ✅ Do not allow bypassing the above settings
   - ✅ Restrict who can push to matching branches (optional)

## Benefits

- **Quality Control**: All code is reviewed before merging
- **Prevent Accidents**: No accidental pushes to main
- **Audit Trail**: All changes are tracked through PRs
- **Collaboration**: Facilitates team review and discussion
