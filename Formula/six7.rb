class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.33"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.33"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.33/six7-0.7.33-arm64-apple-darwin.tar.gz"
      sha256 "0fa9a683ec9b4f7f329091a84bd593eaa534d0d2ccd4d1963317cc2288690a7d"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
