load("@prelude//platforms:defs.bzl", "execution_platform")

execution_platform(
    name = "linux-x86_64",
    cpu_configuration = "config//cpu:x86_64",
    os_configuration = "config//os:linux",
    use_windows_path_separators = False
)

execution_platform(
    name = "macos-arm64",
    cpu_configuration = "config//cpu:arm64",
    os_configuration = "config//os:macos",
    use_windows_path_separators = False
)
