help:
	@echo "\nAvailable commnads:"
	@echo ">> init : Initiates python environment" | sed 's/^/   /'
	@echo ">> clean : removes all pycaches" | sed 's/^/   /'
	@echo ">> commit : commits all changes to git" | sed 's/^/   /'
	@echo ">> release : Updates version, builds and releases package to Pypy" | sed 's/^/   /'

commit:
	@git add -A
	@DESCRIPTION=$$(gum write --width 60 --height 6 --base.margin "1 1" --cursor.foreground 31 --placeholder "Details of this change (CTRL+D to finish)");\
	gum confirm --selected.background 31 "Commit changes?" && git commit -m "$$DESCRIPTION"
	@BRANCH=$$(git rev-parse --abbrev-ref HEAD);\
	git push origin $$BRANCH

