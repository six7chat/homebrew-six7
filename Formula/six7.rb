class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.66"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.66"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.66/six7-0.7.66-arm64-apple-darwin.tar.gz"
      sha256 "2fcd195617962cc470ab0170bfc760501afa546fa0eae154f11df59771714e10"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
