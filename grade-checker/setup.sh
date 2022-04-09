#!/bin/sh
echo "Enter domain(example: studentvue.school.edu): "
read domain
echo "Enter username: "
read username
echo "Enter password: "
read password

sed -i "s/replaceWithDomain/$domain/g" grade-checker
sed -i "s/replaceWithUsername/$username/g" grade-checker
sed -i "s/replaceWithPassword/$password/g" grade-checker

echo "Set domain to $domain"
echo "Set username to $username"
echo "Set password to $password"

su="doas"
sudo -V
if [ $? -eq 0 ]; then
	echo "found sudo, using sudo to copy"
	su="sudo"
else
	echo "failed to find sudo, using doas"
fi
echo "Copying file to /usr/bin/local"
$su cp grade-checker /usr/local/bin/
