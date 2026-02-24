class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.41"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.41"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.41/six7-0.7.41-arm64-apple-darwin.tar.gz"
      sha256 "74ed04a7bb67160a54ff0fa62d310c505cbb9ac1fba055e85293bfef8abbcaa1"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
