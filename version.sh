VERSION=${VERSION:-0.0.0}

case "${1}" in
	master)
		if git log --format=%s -n 1 | grep -i "Merge branch 'major/" > /dev/null; then
			export VERSION_BUMP=major
		elif git log --format=%s -n 1 | grep -i "Merge branch 'minor/" > /dev/null; then
			export VERSION_BUMP=minor
		else
			export VERSION_BUMP=patch
		fi
		echo $(semver -i ${VERSION_BUMP} ${VERSION%%-*})
		;;
	branches|tags|*)
		echo ${VERSION}
		;;
esac
