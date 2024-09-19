import os
import shutil
import subprocess
import time

TEST_WORKSPACE = "test_workspace"

def cleanup():
    """ Helper function to clean up the test workspace after each test. """
    if os.path.exists(TEST_WORKSPACE):
        shutil.rmtree(TEST_WORKSPACE)

def assert_file_exists(path):
    """ Helper function to check if a file exists. """
    assert os.path.exists(path), f"Expected file '{path}' does not exist."

def run_command(command, check=True):
    """ Helper function to run a shell command and print output. """
    result = subprocess.run(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if check and result.returncode != 0:
        raise RuntimeError(f"Command failed: {command}\n{result.stderr}")
    print(result.stdout)
    return result

def test_basic_workspace_creation():
    """ Test to ensure that a basic workspace and crate generation works. """
    # Clean up any existing test workspace
    cleanup()

    # Run the command to create a basic workspace and a crate
    command = f"./target/debug/crust-trust {TEST_WORKSPACE} ui:druid,gtk,piet"
    run_command(command)

    # Wait a moment to ensure the process completes
    time.sleep(2)

    # Check if the workspace Cargo.toml exists
    assert_file_exists(os.path.join(TEST_WORKSPACE, "Cargo.toml"))

    # Check if the `ui` crate was created and contains a Cargo.toml file
    ui_crate_path = os.path.join(TEST_WORKSPACE, "ui")
    assert_file_exists(os.path.join(ui_crate_path, "Cargo.toml"))
    assert_file_exists(os.path.join(ui_crate_path, "src/lib.rs"))

    # Optionally run cargo check to validate that the project is correct
    run_command(f"cargo check --manifest-path {TEST_WORKSPACE}/Cargo.toml")

def test_multiple_crates():
    """ Test to ensure multiple crates are created correctly. """
    # Clean up any existing test workspace
    cleanup()

    # Run the command to create a workspace with multiple crates
    command = f"./target/debug/crust-trust {TEST_WORKSPACE} core:serde,mongodb storage:sysinfo,zfs"
    run_command(command)

    # Wait a moment to ensure the process completes
    time.sleep(2)

    # Check if the workspace Cargo.toml exists
    assert_file_exists(os.path.join(TEST_WORKSPACE, "Cargo.toml"))

    # Check if the `core` and `storage` crates were created
    core_crate_path = os.path.join(TEST_WORKSPACE, "core")
    storage_crate_path = os.path.join(TEST_WORKSPACE, "storage")
    assert_file_exists(os.path.join(core_crate_path, "Cargo.toml"))
    assert_file_exists(os.path.join(core_crate_path, "src/lib.rs"))
    assert_file_exists(os.path.join(storage_crate_path, "Cargo.toml"))
    assert_file_exists(os.path.join(storage_crate_path, "src/lib.rs"))

    # Optionally run cargo check to validate that the project is correct
    run_command(f"cargo check --manifest-path {TEST_WORKSPACE}/Cargo.toml")

if __name__ == "__main__":
    try:
        test_basic_workspace_creation()
        test_multiple_crates()
        print("All tests passed successfully!")
    finally:
        cleanup()
