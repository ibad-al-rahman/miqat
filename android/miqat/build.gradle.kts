import com.android.build.gradle.internal.tasks.factory.dependsOn
import org.gradle.configurationcache.extensions.capitalized

plugins {
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinAndroid)
    alias(libs.plugins.cargoNdk)
    alias(libs.plugins.mavenPublish)
}

android {
    namespace = GradleConfigs.subNamespace("miqat")
    compileSdk = GradleConfigs.compileSdk
    ndkVersion = GradleConfigs.ndkVersion

    defaultConfig {
        minSdk = GradleConfigs.minSdk
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }
}

dependencies {
    // JNA must be the Android @aar variant, which ships jni/<abi>/libjnidispatch.so.
    // The plain jar only bundles desktop/server natives and crashes on Android with
    // UnsatisfiedLinkError. Version catalogs can't express the @aar artifact type, so
    // the coordinate is reconstructed inline while the version stays in the catalog.
    // Declared as implementation (runtime-only) so consumers resolve JNA transitively
    // without having to declare it themselves.
    implementation("net.java.dev.jna:jna:${libs.versions.jna.get()}@aar")
    implementation(libs.core.ktx)
}

cargoNdk {
    module = ".."
    librariesNames = arrayListOf("libmiqat.so")

    // Release .so recompiles std with panic=abort (matches the panic="abort"
    // release profile in miqat_rslib/Cargo.toml). -Zbuild-std needs nightly,
    // selected via RUSTUP_TOOLCHAIN. Debug stays on stable for fast iteration.
    // The "release"/"debug" build types are pre-created by the plugin, so we
    // configure the existing one rather than creating a new one.
    buildTypes {
        getByName("release") {
            extraCargoBuildArguments = arrayListOf("-Z", "build-std=std,panic_abort")
            extraCargoEnv = mapOf(
                "RUSTUP_TOOLCHAIN" to "nightly",
                "RUSTFLAGS" to "-Zunstable-options -Cpanic=immediate-abort"
            )
        }
    }
}

afterEvaluate {
    android.libraryVariants.all { variant ->
        // Resolve buildDirectory to an absolute path. Interpolating the
        // DirectoryProperty directly yields the literal "property 'buildDir'",
        // which the generate/copy Exec tasks (run from the repo root) would
        // create as a stray directory there.
        val outDir = "${layout.buildDirectory.get().asFile.absolutePath}/generated/source/uniffi/${variant.name}/java"

        val generateBindings = tasks.register(
            name = "generate${variant.name.capitalized()}UniFFIBindings",
            type = Exec::class
        ) {
            workingDir = file("../..")
            commandLine(
                "cargo", "run", "-p", "uniffi-bindgen", "generate",
                "--library", "./android/miqat/src/main/jniLibs/arm64-v8a/libmiqat.so",
                "--language", "kotlin",
                "--out-dir", outDir
            )
            dependsOn("buildCargoNdk${variant.name.capitalized()}")
        }

        val copyBindings = tasks.register(
            name = "copy${variant.name.capitalized()}UniFFIBindings",
            type = Exec::class
        ) {
            workingDir = file("../..")
            commandLine("cp", "-r", outDir, "${projectDir}/src/main/")
            dependsOn(generateBindings)
        }
        variant.javaCompileProvider.dependsOn(copyBindings)
        tasks.named("compile${variant.name.capitalized()}Kotlin") { dependsOn(generateBindings) }
        tasks.named("connectedDebugAndroidTest").configure { dependsOn(generateBindings) }
        true
    }
}

mavenPublishing {
    publishToMavenCentral()

    // Only sign when a signing key is configured (in-memory key for CI, or a
    // signing.* keyring locally). This keeps `publishToMavenLocal` dry runs
    // working without GPG, while Central releases still get signed once
    // credentials are present. Central rejects unsigned publications.
    if (project.hasProperty("signingInMemoryKey") || project.hasProperty("signing.keyId")) {
        signAllPublications()
    }

    coordinates(GradleConfigs.mavenGroup, "miqat", GradleConfigs.packageVersion)

    pom {
        name.set("Miqat")
        description.set("High-precision Islamic prayer time calculation library.")
        inceptionYear.set(GradleConfigs.inceptionYear)
        url.set(GradleConfigs.projectUrl)
        licenses {
            license {
                name.set("The Apache License, Version 2.0")
                url.set("http://www.apache.org/licenses/LICENSE-2.0.txt")
            }
        }
        developers {
            developer {
                id.set("Ghamza-Jd")
                name.set("Ghamza-Jd")
                url.set("https://github.com/Ghamza-Jd")
            }
        }
        scm {
            url.set(GradleConfigs.projectUrl)
            connection.set("scm:git:git://github.com/ibad-al-rahman/miqat.git")
            developerConnection.set("scm:git:ssh://git@github.com/ibad-al-rahman/miqat.git")
        }

        // Gradle drops the artifact type of @aar dependencies from the generated
        // POM (gradle/gradle#3170), so Maven-based consumers would otherwise fetch
        // the plain JNA jar (no Android natives). Re-add <type>aar</type> to the
        // JNA dependency so pure-POM consumers also resolve the Android variant.
        // Gradle consumers already resolve it correctly via module metadata.
        // Idempotent: newer JNA releases already emit <type>aar</type> from their
        // own module metadata, so only append when it's missing — otherwise the POM
        // gets a duplicated <type> tag and publication fails ("Duplicated tag: type").
        withXml {
            val dependencies = asNode().children()
                .filterIsInstance<groovy.util.Node>()
                .firstOrNull { (it.name() as? groovy.namespace.QName)?.localPart == "dependencies" }
            val jna = dependencies?.children()
                ?.filterIsInstance<groovy.util.Node>()
                ?.firstOrNull { dependency ->
                    fun text(name: String) =
                        (dependency.get(name) as? groovy.util.NodeList)?.text()
                    text("groupId") == "net.java.dev.jna" && text("artifactId") == "jna"
                }
            val hasType = (jna?.get("type") as? groovy.util.NodeList)?.isNotEmpty() == true
            if (jna != null && !hasType) {
                jna.appendNode("type", "aar")
            }
        }
    }
}
