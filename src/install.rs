/*

Install all the dependencies listed within
package.json in the local node_modules folder.

If tyr.lock is present and is enough to satisfy all the dependencies listed in package.json, the exact versions recorded in tyr.lock are installed, and tyr.lock will be unchanged. tyr will not check for newer versions.

If tyr.lock is absent, or is not enough to satisfy all the dependencies listed in package.json (for example, if you manually add a dependency to package.json), Yarn looks for the newest versions available that satisfy the constraints in package.json. The results are written to yarn.lock.

*/
