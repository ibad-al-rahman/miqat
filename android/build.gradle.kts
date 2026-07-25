// Top-level build file where you can add configuration options common to all sub-projects/modules.
@Suppress("DSL_SCOPE_VIOLATION") // TODO: Remove once KTIJ-19369 is fixed
plugins {
    alias(libs.plugins.kotlinAndroid) apply false
    alias(libs.plugins.androidLibrary) apply false
    alias(libs.plugins.cargoNdk) apply false
    alias(libs.plugins.mavenPublish) apply false
}

allprojects {
    group = GradleConfigs.mavenGroup
    version = GradleConfigs.packageVersion
}
