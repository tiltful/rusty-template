print("✅ Starting project generation...");

let project_name = variable::get("project-name");
let github_user = variable::get("github_username");
let author = variable::get("author_name");

// Update cliff.toml if exists
let cliff_content = "";
try {
    cliff_content = file::read("cliff.toml");
    let updated_content = cliff_content
        .replace("<GITHUB_USER>", github_user)
        .replace("<PROJECT_NAME>", project_name);

    write_file("cliff.toml", updated_content);
    print("📝 Updated cliff.toml with project details");
} catch {
    print("⚠️ cliff.toml not found, skipping...");
}

// Update GitHub workflows
let github_files = [
    ".github/workflows/cd.yml",
    ".github/workflows/ci.yml",
    ".github/ISSUE_TEMPLATE/config.yml",
    ".github/ISSUE_TEMPLATE/bug_report.md",
    ".github/ISSUE_TEMPLATE/feature_request.md"
];

for file_path in github_files {
    try {
        let content = file::read(file_path);
        let updated_content = content
            .replace("{{project-name}}", project_name)
            .replace("<GITHUB_USER>", github_user)
            .replace("<PROJECT_NAME>", project_name)
            .replace("<CRATE>", project_name)
            .replace("rusty-tempate", project_name);

        file::write(file_path, updated_content);
        print("📝 Updated " + file_path);
    } catch {
        print("⚠️ " + file_path + " not found, skipping...");
    }
}

// Handle license file content based on selection
if variable::get("license") == "Apache-2.0" {
    let apache_content = `
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/
   ...
   Copyright 2025 ` + variable::get("author_name") + `

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
`;
    file::write("LICENSE", apache_content);

} else if variable::get("license") == "MIT" {
    let mit_content = `
Copyright 2025 ` + variable::get("author_name") + `

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
`;
    file::write("LICENSE", mit_content);

} else if variable::get("license") == "GPL-3.0" {
    let gpl_content = `
                    GNU GENERAL PUBLIC LICENSE
                       Version 3, 29 June 2007

    Copyright (C) 2025  ` + variable::get("author_name") + `
    Everyone is permitted to copy and distribute verbatim copies
    of this license document, but changing it is not allowed.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
`;
    file::write("LICENSE", gpl_content);

} else if variable::get("license") == "BSD-3-Clause" {
    let bsd_content = `
Copyright 2025 ` + variable::get("author_name") + `
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
`;
    file::write("LICENSE", bsd_content);
}
