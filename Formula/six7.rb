class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.15"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.15"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.15/six7-0.7.15-arm64-apple-darwin.tar.gz"
      sha256 "6eefa7765164e57788219e3f43c1fcd1a2cd4fb85a2d4a9195544e8750838260"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
