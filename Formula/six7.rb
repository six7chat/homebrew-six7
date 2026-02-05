class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.10"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.10"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.10/six7-0.7.10-arm64-apple-darwin.tar.gz"
      sha256 "868244c5de393dcfaab3f7e5ba301adddbcb4eb0e5dbf3f400aa6d441d6bf84e"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
