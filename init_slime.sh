# file for quickly initializing project using "slime" crate
echo "creating necessary folders"
mkdir data
mkdir generated
mkdir templates
# get deploying script
wget "https://raw.githubusercontent.com/jaroslaw-weber/slime/master/deploy_scripts/deploy_template.sh" 
mv "deploy_template.sh" "deploy.sh"
# get travis script
wget "https://raw.githubusercontent.com/jaroslaw-weber/slime/master/deploy_scripts/travis_template.yml"
mv "travis_template.yml" ".travis.yml"